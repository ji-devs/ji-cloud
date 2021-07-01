Some notes:

* Some modules like cover don't need a choose page
* Steps need to be changed at the top level, and pushed to history, but passed down read-only to the app
* However, because we have to send the init args before getting the init result, it's send as a Mutable (app should convert to read_only())
* Themes in terms of the module override is solely within the app (both changing and pushing to history) 
* Jig Theme Id needs to be changed at the top level, and from within the app, but is not pushed to history
* Due to the ubiquity of needing a read-only signal of the calculated ThemeId, a ReadOnlyMutable<ThemeId> is passed down
* Saving, loading, restoring is handled automatically via history changes
* We don't rely on Signals for history anywhere because interim values may be skipped
* "Base" on the app side is generally created before the sub-sections and shared to them all
* BaseInitFromRawArgs is used to create top-level things that are passed down to the app. Some of them may be shared in the top-level app too
