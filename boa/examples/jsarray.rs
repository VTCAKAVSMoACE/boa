// This example goes into the details on how to pass closures as functions
// inside Rust and call them from Javascript.

use boa::{object::JsArray, Context, JsValue};

fn main() -> Result<(), JsValue> {
    // We create a new `Context` to create a new Javascript executor.
    let context = &mut Context::default();

    // Create a new array with 2 elements.
    //
    // [ "Hello, world", true ]
    let array = JsArray::from_list(&[JsValue::new("Hello, world"), JsValue::new(true)], context);

    assert!(!array.is_empty(context)?);

    assert_eq!(array.pop(context)?, JsValue::new(true)); // [ "Hello, world" ]
    assert_eq!(array.pop(context)?, JsValue::new("Hello, world")); // [ ]
    assert_eq!(array.pop(context)?, JsValue::undefined()); // [ ]

    array.push(1, context)?; // [ 1 ]

    assert_eq!(array.pop(context)?, JsValue::new(1)); // [ ]
    assert_eq!(array.pop(context)?, JsValue::undefined()); // [ ]

    array.push_items(
        &[
            JsValue::new(10),
            JsValue::new(12),
            JsValue::new(13),
            JsValue::new(14),
        ],
        context,
    )?; // [ 10, 12, 13, 14 ]

    array.reverse(context)?; // [ 14, 13, 12, 10 ]

    assert_eq!(array.index_of(12, None, context)?, Some(2));

    // We can also use JsObject method `.get()` through the Deref trait.
    let element = array.get(0, context)?; // 0
    assert_eq!(element, JsValue::new(14));

    // Join the array with an optional separator (default ",").
    let joined_array = array.join(None, context)?;
    assert_eq!(joined_array, "14,13,12,10");

    array.fill(false, Some(1), Some(3), context)?;

    let joined_array = array.join(Some("::".into()), context)?;
    assert_eq!(joined_array, "14::false::false::10");

    context
        .global_object()
        .set("myArray", array, true, context)?;

    Ok(())
}
