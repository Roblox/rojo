local Log = require(script.Parent.Parent.Log)

--[[
	A bidirectional map between instance IDs and Roblox instances. It lets us
	keep track of every instance we know about.

	TODO: Track ancestry to catch when stuff moves?
]]
local InstanceMap = {}
InstanceMap.__index = InstanceMap

function InstanceMap.new(onInstanceChanged)
	local self = {
		fromIds = {},
		fromInstances = {},
		instancesToSignal = {},
		onInstanceChanged = onInstanceChanged,
	}

	return setmetatable(self, InstanceMap)
end

--[[
	Disconnect all connections and release all instance references.
]]
function InstanceMap:stop()
	-- I think this is safe.
	for instance in pairs(self.fromInstances) do
		self:removeInstance(instance)
	end
end

function InstanceMap:__fmtDebug(output)
	output:writeLine("InstanceMap {{")
	output:indent()

	-- Collect all of the entries in the InstanceMap and sort them by their
	-- label, which helps make our output deterministic.
	local entries = {}
	for id, instance in pairs(self.fromIds) do
		local label = string.format("%s (%s)", instance:GetFullName(), instance.ClassName)

		table.insert(entries, {id, label})
	end

	table.sort(entries, function(a, b)
		return a[2] < b[2]
	end)

	for _, entry in ipairs(entries) do
		output:writeLine("{}: {}", entry[1], entry[2])
	end

	output:unindent()
	output:write("}")
end

function InstanceMap:insert(id, instance)
	self.fromIds[id] = instance
	self.fromInstances[instance] = id
	self:__connectSignals(instance)
end

function InstanceMap:removeId(id)
	local instance = self.fromIds[id]

	if instance ~= nil then
		self:__disconnectSignals(instance)
		self.fromIds[id] = nil
		self.fromInstances[instance] = nil
	else
		Log.warn("Attempted to remove nonexistant ID {}", id)
	end
end

function InstanceMap:removeInstance(instance)
	local id = self.fromInstances[instance]
	self:__disconnectSignals(instance)

	if id ~= nil then
		self.fromInstances[instance] = nil
		self.fromIds[id] = nil
	else
		Log.warn("Attempted to remove nonexistant instance {}", instance)
	end
end

function InstanceMap:destroyInstance(instance)
	local id = self.fromInstances[instance]

	if id ~= nil then
		self:destroyId(id)
	else
		Log.warn("Attempted to destroy untracked instance {}", instance)
	end
end

function InstanceMap:destroyId(id)
	local instance = self.fromIds[id]
	self:removeId(id)

	if instance ~= nil then
		local descendantsToDestroy = {}

		for otherInstance in pairs(self.fromInstances) do
			if otherInstance:IsDescendantOf(instance) then
				table.insert(descendantsToDestroy, otherInstance)
			end
		end

		for _, otherInstance in ipairs(descendantsToDestroy) do
			self:removeInstance(otherInstance)
		end

		instance:Destroy()
	else
		Log.warn("Attempted to destroy nonexistant ID {}", id)
	end
end

function InstanceMap:__connectSignals(instance)
	-- ValueBase instances have an overriden version of the Changed signal that
	-- only detects changes to their Value property.
	--
	-- We can instead connect listener to each individual property that we care
	-- about on those objects (Name and Value) to emulate the same idea.
	if instance:IsA("ValueBase") then
		local signals = {
			instance:GetPropertyChangedSignal("Name"):Connect(function()
				self:__maybeFireInstanceChanged(instance, "Name")
			end),

			instance:GetPropertyChangedSignal("Value"):Connect(function()
				self:__maybeFireInstanceChanged(instance, "Value")
			end),
		}

		self.instancesToSignal[instance] = signals
	else
		self.instancesToSignal[instance] = instance.Changed:Connect(function(propertyName)
			self:__maybeFireInstanceChanged(instance, propertyName)
		end)
	end
end

function InstanceMap:__maybeFireInstanceChanged(instance, propertyName)
	Log.trace("{}.{} changed", instance:GetFullName(), propertyName)

	if self.onInstanceChanged ~= nil then
		self.onInstanceChanged(instance, propertyName)
	end
end

function InstanceMap:__disconnectSignals(instance)
	local signals = self.instancesToSignal[instance]

	if signals ~= nil then
		-- In most cases, we only have a single signal, so we avoid keeping
		-- around the extra table. ValueBase objects force us to use multiple
		-- signals to emulate the Instance.Changed event, however.
		if typeof(signals) == "table" then
			for _, signal in ipairs(signals) do
				signal:Disconnect()
			end
		else
			signals:Disconnect()
		end

		self.instancesToSignal[instance] = nil
	end
end

return InstanceMap