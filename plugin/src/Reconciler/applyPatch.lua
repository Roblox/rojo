--[[
	Apply a patch to the DOM. Returns any portions of the patch that weren't
	possible to apply.

	Patches can come from the server or be generated by the client.
]]

local Log = require(script.Parent.Parent.Parent.Log)

local PatchSet = require(script.Parent.Parent.PatchSet)
local Types = require(script.Parent.Parent.Types)
local invariant = require(script.Parent.Parent.invariant)

local decodeValue = require(script.Parent.decodeValue)
local getProperty = require(script.Parent.getProperty)
local reify = require(script.Parent.reify)
local setProperty = require(script.Parent.setProperty)

local function applyPatch(instanceMap, patch)
	-- Tracks any portions of the patch that could not be applied to the DOM.
	local unappliedPatch = PatchSet.newEmpty()

	for _, removedIdOrInstance in ipairs(patch.removed) do
		if Types.RbxId(removedIdOrInstance) then
			instanceMap:destroyId(removedIdOrInstance)
		else
			instanceMap:destroyInstance(removedIdOrInstance)
		end
	end

	for id, virtualInstance in pairs(patch.added) do
		if instanceMap.fromIds[id] ~= nil then
			-- This instance already exists. We might've already added it in a
			-- previous iteration of this loop, or maybe this patch was not
			-- supposed to list this instance.
			--
			-- It's probably fine, right?
			continue
		end

		-- Find the first ancestor of this instance that is marked for an
		-- addition.
		--
		-- This helps us make sure we only reify each instance once, and we
		-- start from the top.
		while patch.added[virtualInstance.Parent] ~= nil do
			id = virtualInstance.Parent
			virtualInstance = patch.added[id]
		end

		local parentInstance = instanceMap.fromIds[virtualInstance.Parent]

		if parentInstance == nil then
			-- This would be peculiar. If you create an instance with no
			-- parent, were you supposed to create it at all?
			invariant(
				"Cannot add an instance from a patch that has no parent.\nInstance {} with parent {}.\nState: {:#?}",
				id,
				virtualInstance.Parent,
				instanceMap
			)
		end

		local failedToReify = reify(instanceMap, patch.added, id, parentInstance)

		if not PatchSet.isEmpty(failedToReify) then
			Log.debug("Failed to reify as part of applying a patch: {}", failedToReify)
			PatchSet.assign(unappliedPatch, failedToReify)
		end
	end

	for _, update in ipairs(patch.updated) do
		local instance = instanceMap.fromIds[update.id]

		if instance == nil then
			-- We can't update an instance that doesn't exist.
			-- TODO: Should this be an invariant?
			continue
		end

		-- Track any part of this update that could not be applied.
		local unappliedUpdate = {
			id = update.id,
			changedProperties = {},
		}
		local partiallyApplied = false

		if update.changedClassName ~= nil then
			-- TODO: Support changing class name by destroying + recreating.
			unappliedUpdate.changedClassName = update.changedClassName
			partiallyApplied = true
		end

		if update.changedName ~= nil then
			instance.Name = update.changedName
		end

		if update.changedMetadata ~= nil then
			-- TODO: Support changing metadata. This will become necessary when
			-- Rojo persistently tracks metadata for each instance in order to
			-- remove extra instances.
			unappliedUpdate.changedMetadata = update.changedMetadata
			partiallyApplied = true
		end

		if update.changedProperties ~= nil then
			for propertyName, propertyValue in pairs(update.changedProperties) do
				local ok, decodedValue = decodeValue(propertyValue, instanceMap)
				if not ok then
					unappliedUpdate.changedProperties[propertyName] = propertyValue
					partiallyApplied = true
					continue
				end

				local ok, existingValue = getProperty(instance, propertyName)
				if not ok then
					unappliedUpdate.changedProperties[propertyName] = propertyValue
					partiallyApplied = true
				end

				-- If the existing value is the same, we can skip trying to
				-- apply it. This check is important specifically for very long
				-- string properties. Even if the value we're trying to set is
				-- the same as the existing value, if it is too long, Roblox
				-- will throw an error.
				if decodedValue == existingValue then
					continue
				end

				local ok = setProperty(instance, propertyName, decodedValue)
				if not ok then
					unappliedUpdate.changedProperties[propertyName] = propertyValue
					partiallyApplied = true
				end
			end
		end

		if partiallyApplied then
			table.insert(unappliedPatch.updated, unappliedUpdate)
		end
	end

	return unappliedPatch
end

return applyPatch