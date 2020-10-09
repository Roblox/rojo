return function()
	local applyPatch = require(script.Parent.applyPatch)

	local InstanceMap = require(script.Parent.Parent.InstanceMap)
	local PatchSet = require(script.Parent.Parent.PatchSet)

	local dummy = Instance.new("Folder")
	local function wasDestroyed(instance)
		-- If an instance was destroyed, its parent property is locked.
		local ok = pcall(function()
			local oldParent = instance.Parent
			instance.Parent = dummy
			instance.Parent = oldParent
		end)

		return not ok
	end

	it("should return an empty patch if given an empty patch", function()
		local patch = applyPatch(InstanceMap.new(), PatchSet.newEmpty())
		assert(PatchSet.isEmpty(patch), "expected remaining patch to be empty")
	end)

	it("should destroy instances listed for remove", function()
		local root = Instance.new("Folder")

		local child = Instance.new("Folder")
		child.Name = "Child"
		child.Parent = root

		local instanceMap = InstanceMap.new()
		instanceMap:insert("ROOT", root)
		instanceMap:insert("CHILD", child)

		local patch = PatchSet.newEmpty()
		table.insert(patch.removed, child)

		local unapplied = applyPatch(instanceMap, patch)
		assert(PatchSet.isEmpty(unapplied), "expected remaining patch to be empty")

		assert(not wasDestroyed(root), "expected root to be left alone")
		assert(wasDestroyed(child), "expected child to be destroyed")

		instanceMap:stop()
	end)

	it("should destroy IDs listed for remove", function()
		local root = Instance.new("Folder")

		local child = Instance.new("Folder")
		child.Name = "Child"
		child.Parent = root

		local instanceMap = InstanceMap.new()
		instanceMap:insert("ROOT", root)
		instanceMap:insert("CHILD", child)

		local patch = PatchSet.newEmpty()
		table.insert(patch.removed, "CHILD")

		local unapplied = applyPatch(instanceMap, patch)
		assert(PatchSet.isEmpty(unapplied), "expected remaining patch to be empty")
		expect(instanceMap:size()).to.equal(1)

		assert(not wasDestroyed(root), "expected root to be left alone")
		assert(wasDestroyed(child), "expected child to be destroyed")

		instanceMap:stop()
	end)

	it("should add instances to the DOM", function()
		-- Many of the details of this functionality are instead covered by
		-- tests on reify, not here.

		local root = Instance.new("Folder")

		local instanceMap = InstanceMap.new()
		instanceMap:insert("ROOT", root)

		local patch = PatchSet.newEmpty()
		patch.added["CHILD"] = {
			Id = "CHILD",
			ClassName = "Model",
			Name = "Child",
			Parent = "ROOT",
			Children = {"GRANDCHILD"},
			Properties = {},
		}

		patch.added["GRANDCHILD"]  = {
			Id = "GRANDCHILD",
			ClassName = "Part",
			Name = "Grandchild",
			Parent = "CHILD",
			Children = {},
			Properties = {},
		}

		local unapplied = applyPatch(instanceMap, patch)
		assert(PatchSet.isEmpty(unapplied), "expected remaining patch to be empty")
		expect(instanceMap:size()).to.equal(3)

		local child = root:FindFirstChild("Child")
		expect(child).to.be.ok()
		expect(child.ClassName).to.equal("Model")
		expect(child).to.equal(instanceMap.fromIds["CHILD"])

		local grandchild = child:FindFirstChild("Grandchild")
		expect(grandchild).to.be.ok()
		expect(grandchild.ClassName).to.equal("Part")
		expect(grandchild).to.equal(instanceMap.fromIds["GRANDCHILD"])
	end)

	it("should return unapplied additions when instances cannot be created", function()
		local root = Instance.new("Folder")

		local instanceMap = InstanceMap.new()
		instanceMap:insert("ROOT", root)

		local patch = PatchSet.newEmpty()
		patch.added["OOPSIE"] = {
			Id = "OOPSIE",
			-- Hopefully Roblox never makes an instance with this ClassName.
			ClassName = "UH OH",
			Name = "FUBAR",
			Parent = "ROOT",
			Children = {},
			Properties = {},
		}

		local unapplied = applyPatch(instanceMap, patch)
		expect(unapplied.added["OOPSIE"]).to.equal(patch.added["OOPSIE"])
		expect(instanceMap:size()).to.equal(1)
		expect(#root:GetChildren()).to.equal(0)
	end)
end