Some notes:

* Some modules like cover don't need a choose page
* Steps need to be changed at the top level, but passed down read-only to the app
* Saving, loading, restoring is handled automatically via history changes, but the consumer is responsible for pushing new history
* The exception is Steps which are pushed at this level (because this is where it's imperatively set)
* Themes are not an exception, even though a mutable is passed down, because it's changed from the consumer
  (we don't rely on signals for history because interim values may be skipped)
* On the consumer side - "Base" is generally created before the sub-sections and shared to them all
* BaseInitFromRawArgs is used to create top-level things that are passed down to the consumer. Some of them may be shared in the top-level app too