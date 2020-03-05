# Rojo Changelog

## Unreleased Changes

## [0.5.4](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.4) (February 26, 2020)
This is a general maintenance release for the Rojo 0.5.x release series.

* Updated reflection database and other dependencies.
* First stable release with binaries for macOS and Linux.

## [0.6.0 Alpha 1](https://github.com/rojo-rbx/rojo/releases/tag/v0.6.0-alpha.1) (January 22, 2020)

### General
* Added support for nested project files. ([#95](https://github.com/rojo-rbx/rojo/issues/95))
* Added project file hot-reloading. ([#10](https://github.com/rojo-rbx/rojo/issues/10)])
* Fixed Rojo dropping Ref properties ([#142](https://github.com/rojo-rbx/rojo/issues/142))
    * This means that properties like `PrimaryPart` now work!
* Improved live sync protocol to reduce round-trips and improve syncing consistency.
* Improved support for binary model files and places.

### Command Line
* Added `--verbose`/`-v` flag, which can be specified multiple times to increase verbosity.
* Added support for automatically finding Roblox Studio's auth cookie for `rojo upload` on Windows.
* Added support for building, serving and uploading sources that aren't Rojo projects.
* Improved feedback from `rojo serve`.
* Removed support for legacy `roblox-project.json` projects, deprecated in an early Rojo 0.5.0 alpha.
* Rojo no longer traverses directories upwards looking for project files.
    * Though undocumented, Rojo 0.5.x will search for a project file contained in any ancestor folders. This feature was removed to better support other 0.6.x features.

### Roblox Studio Plugin
* Added "connecting" state to improve experience when live syncing.
* Added "error" state to show errors in a place that isn't the output panel.
* Improved diagnostics for when the Rojo plugin cannot create an instance.

## [0.5.3](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.3) (October 15, 2019)
* Fixed an issue where Rojo would throw an error when encountering recently-added instance classes.

## [0.5.2](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.2) (October 14, 2019)
* Fixed an issue where `LocalizationTable` instances would have their column order randomized. ([#173](https://github.com/rojo-rbx/rojo/issues/173))

## [0.5.1](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.1) (October 4, 2019)
* Fixed an issue where Rojo would drop changes if they happened too quickly ([#252](https://github.com/rojo-rbx/rojo/issues/252))
* Improved diagnostics for when the Rojo plugin cannot create an instance.
* Updated dependencies
    * This brings Rojo's reflection database from client release 395 to client release 404.

## [0.5.0](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0) (August 27, 2019)
* Changed `.model.json` naming, which may require projects to migrate ambiguous cases:
    * The file name now takes precedence over the `Name` field in the model, like Rojo 0.4.x.
    * The `Name` field of the top-level instance is now optional. It's recommended that you remove it from your models.
    * Rojo will emit a warning when `Name` is specified and does not match the name from the file.
* Fixed `Rect` values being set to `0, 0, 0, 0` when synced with the Rojo plugin. ([#201](https://github.com/rojo-rbx/rojo/issues/201))
* Fixed live-syncing of `PhysicalProperties`, `NumberSequence`, and `ColorSequence` values

## [0.5.0 Alpha 13](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.13) (August 2, 2019)
* Bumped minimum Rust version to 1.34.0.
* Fixed default port documentation in `rojo serve --help` ([#219](https://github.com/rojo-rbx/rojo/issues/219))
* Fixed BrickColor support by upgrading Roblox-related dependencies

## [0.5.0 Alpha 12](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.12) (July 2, 2019)
* Added `.meta.json` files
    * `init.meta.json` files replace `init.model.json` files from Rojo 0.4.x ([#183](https://github.com/rojo-rbx/rojo/pull/183))
    * Other `.meta.json` files allow attaching extra data to other files ([#189](https://github.com/rojo-rbx/rojo/pull/189))
* Added support for infinite and NaN values in types like `Vector2` when building models and places.
    * These types aren't supported for live-syncing yet due to limitations around JSON encoding.
* Added support for using `SharedString` values when building XML models and places.
* Added support for live-syncing `CollectionService` tags.
* Added a warning when building binary place files, since they're still experimental and have bugs.
* Added a warning when trying to use Rojo 0.5.x with a Rojo 0.4.x-only project.
* Added a warning when a Rojo project contains keys that start with `$`, which are reserved names. ([#191](https://github.com/rojo-rbx/rojo/issues/191))
* Rojo now throws an error if unknown keys are found most files.
* Added an icon to the plugin's toolbar button
* Changed the plugin to use a docking widget for all UI.
* Changed the plugin to ignore unknown properties when live-syncing.
    * Rojo's approach to this problem might change later, like with a strict model mode ([#190](https://github.com/rojo-rbx/rojo/issues/190)) or another approach.
* Upgraded to reflection database from client release 388.
* Updated Rojo's branding to shift the color palette to make it work better on dark backgrounds

## [0.5.0 Alpha 11](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.11) (May 29, 2019)
* Added support for implicit property values in JSON model files ([#154](https://github.com/rojo-rbx/rojo/pull/154))
* `Content` propertyes can now be specified in projects and model files as regular string literals.
* Added support for `BrickColor` properties.
* Added support for properties added in client release 384, like `Lighting.Technology` being set to `"ShadowMap"`.
* Improved performance when working with XML models and places
* Fixed serializing empty `Content` properties as XML
* Fixed serializing infinite and NaN floating point properties in XML
* Improved compatibility with XML models
* Plugin should now be able to live-sync more properties, and ignore ones it can't, like `Lighting.Technology`.

## 0.5.0 Alpha 10
* This release was a dud due to [issue #176](https://github.com/rojo-rbx/rojo/issues/176) and was rolled back.

## [0.5.0 Alpha 9](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.9) (April 4, 2019)
* Changed `rojo build` to use buffered I/O, which can make it up to 2x faster in some cases.
    * Building [*Road Not Taken*](https://github.com/rojo-rbx/roads) to an `rbxlx` file dropped from 150ms to 70ms on my machine
* Fixed `LocalizationTable` instances being made from `csv` files incorrectly interpreting empty rows and columns. ([#149](https://github.com/rojo-rbx/rojo/pull/149))
* Fixed CSV files with entries that parse as numbers causing Rojo to panic. ([#152](https://github.com/rojo-rbx/rojo/pull/152))
* Improved error messages when malformed CSV files are found in a Rojo project.

## [0.5.0 Alpha 8](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.8) (March 29, 2019)
* Added support for a bunch of new types when dealing with XML model/place files:
    * `ColorSequence`
    * `Float64`
    * `Int64`
    * `NumberRange`
    * `NumberSequence`
    * `PhysicalProperties`
    * `Ray`
    * `Rect`
    * `Ref`
* Improved server instance ordering behavior when files are added during a live session ([#135](https://github.com/rojo-rbx/rojo/pull/135))
* Fixed error being thrown when trying to unload the Rojo plugin.
* Added partial fix for [issue #141](https://github.com/rojo-rbx/rojo/issues/141) for `Lighting.Technology`, which should restore live sync functionality for the default project file.

## [0.5.0 Alpha 6](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.6) (March 19, 2019)
* Fixed `rojo init` giving unexpected results by upgrading to `rbx_dom_weak` 1.1.0
* Fixed live server not responding when the Rojo plugin is connected ([#133](https://github.com/rojo-rbx/rojo/issues/133))
* Updated default place file:
    * Improved default properties to be closer to Studio's built-in 'Baseplate' template
    * Added a baseplate to the project file (Thanks, [@AmaranthineCodices](https://github.com/AmaranthineCodices/)!)
* Added more type support to Rojo plugin
* Fixed some cases where the Rojo plugin would leave around objects that it knows should be deleted
* Updated plugin to correctly listen to `Plugin.Unloading` when installing or uninstalling new plugins

## [0.5.0 Alpha 5](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.5) (March 1, 2019)
* Upgraded core dependencies, which improves compatibility for lots of instance types
    * Upgraded from `rbx_tree` 0.2.0 to `rbx_dom_weak` 1.0.0
    * Upgraded from `rbx_xml` 0.2.0 to `rbx_xml` 0.4.0
    * Upgraded from `rbx_binary` 0.2.0 to `rbx_binary` 0.4.0
* Added support for non-primitive types in the Rojo plugin.
    * Types like `Color3` and `CFrame` can now be updated live!
* Fixed plugin assets flashing in on first load ([#121](https://github.com/rojo-rbx/rojo/issues/121))
* Changed Rojo's HTTP server from Rouille to Hyper, which reduced the release size by around a megabyte.
* Added property type inference to projects, which makes specifying services a lot easier ([#130](https://github.com/rojo-rbx/rojo/pull/130))
* Made error messages from invalid and missing files more user-friendly

## [0.5.0 Alpha 4](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.4) (February 8, 2019)
* Added support for nested partitions ([#102](https://github.com/rojo-rbx/rojo/issues/102))
* Added support for 'transmuting' partitions ([#112](https://github.com/rojo-rbx/rojo/issues/112))
* Added support for aliasing filesystem paths ([#105](https://github.com/rojo-rbx/rojo/issues/105))
* Changed Windows builds to statically link the CRT ([#89](https://github.com/rojo-rbx/rojo/issues/89))

## [0.5.0 Alpha 3](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.3) (February 1, 2019)
* Changed default project file name from `roblox-project.json` to `default.project.json` ([#120](https://github.com/rojo-rbx/rojo/pull/120))
    * The old file name will still be supported until 0.5.0 is fully released.
* Added warning when loading project files that don't end in `.project.json`
    * This new extension enables Rojo to distinguish project files from random JSON files, which is necessary to support nested projects.
* Added new (empty) diagnostic page served from the server
* Added better error messages for when a file is missing that's referenced by a Rojo project
* Added support for visualization endpoints returning GraphViz source when Dot is not available
* Fixed an in-memory filesystem regression introduced recently ([#119](https://github.com/rojo-rbx/rojo/pull/119))

## [0.5.0 Alpha 2](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.2) (January 28, 2019)
* Added support for `.model.json` files, compatible with 0.4.x
* Fixed in-memory filesystem not handling out-of-order filesystem change events
* Fixed long-polling error caused by a promise mixup ([#110](https://github.com/rojo-rbx/rojo/issues/110))

## [0.5.0 Alpha 1](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.1) (January 25, 2019)
* Changed plugin UI to be way prettier
    * Thanks to [Reselim](https://github.com/Reselim) for the design!
* Changed plugin error messages to be a little more useful
* Removed unused 'Config' button in plugin UI
* Fixed bug where bad server responses could cause the plugin to be in a bad state
* Upgraded to rbx\_tree, rbx\_xml, and rbx\_binary 0.2.0, which dramatically expands the kinds of properties that Rojo can handle, especially in XML.

## [0.5.0 Alpha 0](https://github.com/rojo-rbx/rojo/releases/tag/v0.5.0-alpha.0) (January 14, 2019)
* "Epiphany" rewrite, in progress since the beginning of time
* New live sync protocol
    * Uses HTTP long polling to reduce request count and improve responsiveness
* New project format
    * Hierarchical, preventing overlapping partitions
* Added `rojo build` command
    * Generates `rbxm`, `rbxmx`, `rbxl`, or `rbxlx` files out of your project
    * Usage: `rojo build <PROJECT> --output <OUTPUT>.rbxm`
* Added `rojo upload` command
    * Generates and uploads a place or model to roblox.com out of your project
    * Usage: `rojo upload <PROJECT> --cookie "<ROBLOSECURITY>" --asset_id <PLACE_ID>`
* New plugin
    * Only one button now, "Connect"
    * New UI to pick server address and port
    * Better error reporting
* Added support for `.csv` files turning into `LocalizationTable` instances
* Added support for `.txt` files turning into `StringValue` instances
* Added debug visualization code to diagnose problems
    * `/visualize/rbx` and `/visualize/imfs` show instance and file state respectively; they require GraphViz to be installed on your machine.
* Added optional place ID restrictions to project files
    * This helps prevent syncing in content to the wrong place
    * Multiple places can be specified, like when building a multi-place game
* Added support for specifying properties on services in project files

## [0.4.13](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.13) (November 12, 2018)
* When `rojo.json` points to a file or directory that does not exist, Rojo now issues a warning instead of throwing an error and exiting

## [0.4.12](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.12) (June 21, 2018)
* Fixed obscure assertion failure when renaming or deleting files ([#78](https://github.com/rojo-rbx/rojo/issues/78))
* Added a `PluginAction` for the sync in command, which should help with some automation scripts ([#80](https://github.com/rojo-rbx/rojo/pull/80))

## [0.4.11](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.11) (June 10, 2018)
* Defensively insert existing instances into RouteMap; should fix most duplication cases when syncing into existing trees.
* Fixed incorrect synchronization from `Plugin:_pull` that would cause polling to create issues
* Fixed incorrect file routes being assigned to `init.lua` and `init.model.json` files
* Untangled route handling-internals slightly

## [0.4.10](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.10) (June 2, 2018)
* Added support for `init.model.json` files, which enable versioning `Tool` instances (among other things) with Rojo. ([#66](https://github.com/rojo-rbx/rojo/issues/66))
* Fixed obscure error when syncing into an invalid service.
* Fixed multiple sync processes occurring when a server ID mismatch is detected.

## [0.4.9](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.9) (May 26, 2018)
* Fixed warning when renaming or removing files that would sometimes corrupt the instance cache ([#72](https://github.com/rojo-rbx/rojo/pull/72))
* JSON models are no longer as strict -- `Children` and `Properties` are now optional.

## [0.4.8](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.8) (May 26, 2018)
* Hotfix to prevent errors from being thrown when objects managed by Rojo are deleted

## [0.4.7](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.7) (May 25, 2018)
* Added icons to the Rojo plugin, made by [@Vorlias](https://github.com/Vorlias)! ([#70](https://github.com/rojo-rbx/rojo/pull/70))
* Server will now issue a warning if no partitions are specified in `rojo serve` ([#40](https://github.com/rojo-rbx/rojo/issues/40))

## [0.4.6](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.6) (May 21, 2018)
* Rojo handles being restarted by Roblox Studio more gracefully ([#67](https://github.com/rojo-rbx/rojo/issues/67))
* Folders should no longer get collapsed when syncing occurs.
* **Significant** robustness improvements with regards to caching.
    * **This should catch all existing script duplication bugs.**
    * If there are any bugs with script duplication or caching in the future, restarting the Rojo server process will fix them for that session.
* Fixed message in plugin not being prefixed with `Rojo: `.

## [0.4.5](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.5) (May 1, 2018)
* Rojo messages are now prefixed with `Rojo: ` to make them stand out in the output more.
* Fixed server to notice file changes *much* more quickly. (200ms vs 1000ms)
* Server now lists name of project when starting up.
* Rojo now throws an error if no project file is found. ([#63](https://github.com/rojo-rbx/rojo/issues/63))
* Fixed multiple sync operations occuring at the same time. ([#61](https://github.com/rojo-rbx/rojo/issues/61))
* Partitions targeting files directly now work as expected. ([#57](https://github.com/rojo-rbx/rojo/issues/57))

## [0.4.4](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.4) (April 7, 2018)
* Fix small regression introduced in 0.4.3

## [0.4.3](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.3) (April 7, 2018)
* Plugin now automatically selects `HttpService` if it determines that HTTP isn't enabled ([#58](https://github.com/rojo-rbx/rojo/pull/58))
* Plugin now has much more robust handling and will wipe all state when the server changes.
    * This should fix issues that would otherwise be solved by restarting Roblox Studio.

## [0.4.2](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.2) (April 4, 2018)
* Fixed final case of duplicated instance insertion, caused by reconciled instances not being inserted into `RouteMap`.
    * The reconciler is still not a perfect solution, especially if script instances get moved around without being destroyed. I don't think this can be fixed before a big refactor.

## [0.4.1](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.1) (April 1, 2018)
* Merged plugin repository into main Rojo repository for easier tracking.
* Improved `RouteMap` object tracking; this should fix some cases of duplicated instances being synced into the tree.

## [0.4.0](https://github.com/rojo-rbx/rojo/releases/tag/v0.4.0) (March 27, 2018)
* Protocol version 1, which shifts more responsibility onto the server
    * This is a **major breaking** change!
    * The server now has a content of 'filter plugins', which transform data at various stages in the pipeline
    * The server now exposes Roblox instance objects instead of file contents, which lines up with how `rojo pack` will work, and paves the way for more robust syncing.
* Added `*.model.json` files, which let you embed small Roblox objects into your Rojo tree.
* Improved error messages in some cases ([#46](https://github.com/rojo-rbx/rojo/issues/46))

## [0.3.2](https://github.com/rojo-rbx/rojo/releases/tag/v0.3.2) (December 20, 2017)
* Fixed `rojo serve` failing to correctly construct an absolute root path when passed as an argument
* Fixed intense CPU usage when running `rojo serve`

## [0.3.1](https://github.com/rojo-rbx/rojo/releases/tag/v0.3.1) (December 14, 2017)
* Improved error reporting when invalid JSON is found in a `rojo.json` project
    * These messages are passed on from Serde

## [0.3.0](https://github.com/rojo-rbx/rojo/releases/tag/v0.3.0) (December 12, 2017)
* Factored out the plugin into a separate repository
* Fixed server when using a file as a partition
    * Previously, trailing slashes were put on the end of a partition even if the read request was an empty string. This broke file reading on Windows when a partition pointed to a file instead of a directory!
* Started running automatic tests on Travis CI (#9)

## [0.2.3](https://github.com/rojo-rbx/rojo/releases/tag/v0.2.3) (December 4, 2017)
* Plugin only release
* Tightened `init` file rules to only match script files
    * Previously, Rojo would sometimes pick up the wrong file when syncing

## [0.2.2](https://github.com/rojo-rbx/rojo/releases/tag/v0.2.2) (December 1, 2017)
* Plugin only release
* Fixed broken reconciliation behavior with `init` files

## [0.2.1](https://github.com/rojo-rbx/rojo/releases/tag/v0.2.1) (December 1, 2017)
* Plugin only release
* Changes default port to 8000

## [0.2.0](https://github.com/rojo-rbx/rojo/releases/tag/v0.2.0) (December 1, 2017)
* Support for `init.lua` like rbxfs and rbxpacker
* More robust syncing with a new reconciler

## [0.1.0](https://github.com/rojo-rbx/rojo/releases/tag/v0.1.0) (November 29, 2017)
* Initial release, functionally very similar to [rbxfs](https://github.com/rojo-rbx/rbxfs)