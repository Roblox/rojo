local Roact = require(script:FindFirstAncestor("Rojo").Roact)

local Plugin = script:FindFirstAncestor("Plugin")

local Assets = require(Plugin.Assets)
local Session = require(Plugin.Session)
local Config = require(Plugin.Config)
local Version = require(Plugin.Version)
local Logging = require(Plugin.Logging)
local DevSettings = require(Plugin.DevSettings)
local preloadAssets = require(Plugin.preloadAssets)

local ConnectPanel = require(Plugin.Components.ConnectPanel)
local ConnectionActivePanel = require(Plugin.Components.ConnectionActivePanel)

local e = Roact.createElement

local function showUpgradeMessage(lastVersion)
	local message = (
		"Rojo detected an upgrade from version %s to version %s." ..
		"\nMake sure you have also upgraded your server!" ..
		"\n\nRojo plugin version %s is intended for use with server version %s."
	):format(
		Version.display(lastVersion), Version.display(Config.version),
		Version.display(Config.version), Config.expectedServerVersionString
	)

	Logging.info(message)
end

--[[
	Check if the user is using a newer version of Rojo than last time. If they
	are, show them a reminder to make sure they check their server version.
]]
local function checkUpgrade(plugin)
	-- When developing Rojo, there's no use in doing version checks
	if DevSettings:isEnabled() then
		return
	end

	local lastVersion = plugin:GetSetting("LastRojoVersion")

	if lastVersion then
		local wasUpgraded = Version.compare(Config.version, lastVersion) == 1

		if wasUpgraded then
			showUpgradeMessage(lastVersion)
		end
	end

	plugin:SetSetting("LastRojoVersion", Config.version)
end

local SessionStatus = {
	Disconnected = "Disconnected",
	Connected = "Connected",
	ConfiguringSession = "ConfiguringSession",
	-- TODO: Error?
}

setmetatable(SessionStatus, {
	__index = function(_, key)
		error(("%q is not a valid member of SessionStatus"):format(tostring(key)), 2)
	end,
})

local App = Roact.Component:extend("App")

function App:init()
	self:setState({
		address = "",
		port = "",
		sessionStatus = SessionStatus.Disconnected,
	})

	self.connectButton = nil
	self.currentSession = nil

	self.displayedVersion = DevSettings:isEnabled()
		and Config.codename
		or Version.display(Config.version)
end

function App:getConnectionPair()
	local address = self.state.address
	if address:len() == 0 then
		address = Config.defaultHost
	end

	local port = self.state.port
	if port:len() == 0 then
		port = Config.defaultPort
	end

	return address, port
end

function App:startSession()
	local address, port = self:getConnectionPair()

	Logging.trace("Starting new session")

	local success, session = Session.new({
		address = address,
		port = port,
		onError = function(message)
			Logging.warn("Rojo session terminated because of an error:\n%s", tostring(message))
			self.currentSession = nil

			self:setState({
				sessionStatus = SessionStatus.Disconnected,
			})
		end
	})

	if success then
		self.currentSession = session
		self:setState({
			sessionStatus = SessionStatus.Connected,
		})
	end
end

function App:stopSession()
	Logging.trace("Disconnecting session")

	self.currentSession:disconnect()
	self.currentSession = nil
	self:setState({
		sessionStatus = SessionStatus.Disconnected,
	})

	Logging.trace("Session terminated by user")
end

function App:render()
	-- FIXME: https://github.com/Roblox/roact/issues/209
	local children = {}

	if self.state.sessionStatus == SessionStatus.Connected then
		children = {
			ConnectionActivePanel = e(ConnectionActivePanel, {
				stopSession = function()
					self:stopSession()
				end,
			}),
		}
	elseif self.state.sessionStatus == SessionStatus.ConfiguringSession then
		children = {
			ConnectPanel = e(ConnectPanel, {
				address = self.state.address,
				port = self.state.port,

				changeAddress = function(address)
					self:setState({ address = address })
				end,
				changePort = function(port)
					self:setState({ port = port })
				end,
				connect = function()
					self:startSession()
				end,
				cancel = function()
					Logging.trace("Canceling session configuration")

					self:setState({
						sessionStatus = SessionStatus.Disconnected,
					})
				end,
			}),
		}
	end

	return e("ScreenGui", {
		AutoLocalize = false,
		ZIndexBehavior = Enum.ZIndexBehavior.Sibling,
	}, children)
end

function App:didMount()
	Logging.trace("Rojo %s initializing", self.displayedVersion)

	local toolbar = self.props.plugin:CreateToolbar("Rojo " .. self.displayedVersion)

	local toggleAction = self.props.plugin:CreatePluginAction("rojo/toggle", "Rojo: Toggle connection",
		"Toggles connection to a running Rojo session")

	toggleAction.Triggered:Connect(function()
		if self.state.sessionStatus == SessionStatus.Connected then
			self:stopSession()
		else
			self:startSession()
		end
	end)

	self.connectButton = toolbar:CreateButton(
		"Connect",
		"Connect to a running Rojo session",
		Assets.StartSession)
	self.connectButton.ClickableWhenViewportHidden = false
	self.connectButton.Click:Connect(function()
		checkUpgrade(self.props.plugin)

		if self.state.sessionStatus == SessionStatus.Connected then
			self:stopSession()
		elseif self.state.sessionStatus == SessionStatus.Disconnected then
			Logging.trace("Starting session configuration")

			self:setState({
				sessionStatus = SessionStatus.ConfiguringSession,
			})
		elseif self.state.sessionStatus == SessionStatus.ConfiguringSession then
			Logging.trace("Canceling session configuration")

			self:setState({
				sessionStatus = SessionStatus.Disconnected,
			})
		end
	end)

	preloadAssets()
end

function App:willUnmount()
	if self.currentSession ~= nil then
		self.currentSession:disconnect()
		self.currentSession = nil
	end
end

function App:didUpdate()
	local connectActive = self.state.sessionStatus == SessionStatus.ConfiguringSession
		or self.state.sessionStatus == SessionStatus.Connected

	self.connectButton:SetActive(connectActive)

	if self.state.sessionStatus == SessionStatus.Connected then
		self.connectButton.Icon = Assets.SessionActive
	else
		self.connectButton.Icon = Assets.StartSession
	end
end

return App