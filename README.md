# Leptos Draggable List Test

This is a test of creating a Drag and Drop capable list using [Leptos](https://github.com/leptos-rs/leptos) web framework in Rust.

DragList takes a `Vec<T> : Listable` of items to list.  The Listable trait just needs to implement get_id and get_name.

### Signals are used:
* DragList contains the itemslist - and provides setters for 3 signals to subitems via provide_context.
* DragItem
	* Contains two internal signals for item state.  is_dragging and is_hoverred_over.  These drive the UI changes (adding classes to display hover & drag states).
	* Uses the 3 setters available from context to pass their item_id back to the DragList - and fire if a drop occurs.
		* JS Event.DataTransfer is used to provide an item_id and fire the drop event.
  

## Testing the project
`cargo leptos watch`
http://127.0.0.1:3000

### Author's Note
This is my first rust and webassembly project.  The code is free to copy and use but I have no idea how "rusty" it is.  It is just a proof of concept/learning experience before I start a _real_ project. 