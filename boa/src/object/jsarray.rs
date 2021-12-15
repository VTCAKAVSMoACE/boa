use std::ops::Deref;

use crate::{
    builtins::Array,
    gc::{Finalize, Trace},
    object::{JsObject, JsObjectType},
    Context, JsResult, JsString, JsValue,
};

/// JavaScript `Array` rust object.
#[derive(Debug, Clone, Trace, Finalize)]
pub struct JsArray {
    inner: JsObject,
}

impl JsArray {
    #[inline]
    pub fn empty(context: &mut Context) -> Self {
        let inner = Array::array_create(0, None, context)
            .expect("creating an empty array with the default prototype must not fail");

        Self { inner }
    }

    #[inline]
    pub fn new<I>(elements: I, context: &mut Context) -> Self
    where
        I: IntoIterator<Item = JsValue>,
    {
        Self {
            inner: Array::create_array_from_list(elements, context),
        }
    }

    #[inline]
    pub fn from(object: JsObject, context: &mut Context) -> JsResult<Self> {
        if object.borrow().is_array() {
            Ok(Self { inner: object })
        } else {
            context.throw_type_error("object is not an Array")
        }
    }

    #[inline]
    pub fn length(&self, context: &mut Context) -> JsResult<usize> {
        self.inner.length_of_array_like(context)
    }

    #[inline]
    pub fn is_empty(&self, context: &mut Context) -> JsResult<bool> {
        self.inner.length_of_array_like(context).map(|len| len == 0)
    }

    #[inline]
    pub fn push<T>(&self, value: T, context: &mut Context) -> JsResult<JsValue>
    where
        T: Into<JsValue>,
    {
        self.push_items(&[value.into()], context)
    }

    #[inline]
    pub fn push_items(&self, items: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Array::push(&self.inner.clone().into(), items, context)
    }

    #[inline]
    pub fn pop(&self, context: &mut Context) -> JsResult<JsValue> {
        Array::pop(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn shift(&self, context: &mut Context) -> JsResult<JsValue> {
        Array::shift(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn unshift(&self, items: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Array::shift(&self.inner.clone().into(), items, context)
    }

    #[inline]
    pub fn reverse(&self, context: &mut Context) -> JsResult<Self> {
        Array::reverse(&self.inner.clone().into(), &[], context)?;
        Ok(self.clone())
    }

    #[inline]
    pub fn join(&self, separator: Option<JsString>, context: &mut Context) -> JsResult<JsString> {
        Array::join(&self.inner.clone().into(), &[separator.into()], context).map(|x| {
            x.as_string()
                .cloned()
                .expect("Array.prototype.join always returns string")
        })
    }

    #[inline]
    pub fn fill<T>(
        &self,
        value: T,
        start: Option<u32>,
        end: Option<u32>,
        context: &mut Context,
    ) -> JsResult<Self>
    where
        T: Into<JsValue>,
    {
        Array::fill(
            &self.inner.clone().into(),
            &[value.into(), start.into(), end.into()],
            context,
        )?;
        Ok(self.clone())
    }

    #[inline]
    pub fn index_of<T>(
        &self,
        search_element: T,
        from_index: Option<u32>,
        context: &mut Context,
    ) -> JsResult<Option<u32>>
    where
        T: Into<JsValue>,
    {
        let index = Array::index_of(
            &self.inner.clone().into(),
            &[search_element.into(), from_index.into()],
            context,
        )?
        .as_number()
        .expect("Array.prototype.indexOf should always return number");

        #[allow(clippy::float_cmp)]
        if index == -1.0 {
            Ok(None)
        } else {
            Ok(Some(index as u32))
        }
    }
}

impl From<JsArray> for JsObject {
    #[inline]
    fn from(o: JsArray) -> Self {
        o.inner.clone()
    }
}

impl From<JsArray> for JsValue {
    #[inline]
    fn from(o: JsArray) -> Self {
        o.inner.clone().into()
    }
}

impl Deref for JsArray {
    type Target = JsObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JsObjectType for JsArray {}
