/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    bezier.rs
@brief   Part of geometry library
 */

//a Notes
//
//

//a Imports
use std::cell::RefCell;

use crate::{BufferClient, ByteBuffer, Renderable};

//a BufferData
//tp BufferData
/// A data buffer for use with OpenGL vertex data. It may be indices
/// or vertex coordinates etc.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  a `View` on the data is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// The data buffer may, indeed, contain data for more than one object
/// - and the objects may have different data per vertex. The data
/// buffer is pretty free-form, it is a `View` on the `Data` which
/// identifies the object it applies to, and the vertex attributes
/// required
///
/// A data buffer may then be used by many `View`s. Each `View` may be
/// used by many primitives for a single model; alternatively,
/// primitives may have their own individual Views.
///
/// Of course the model may be instantiated many times in a single scene.
///
/// OpenGL will have one copy of the data for all the primitives and models.
#[derive(Debug)]
pub struct BufferData<'a, R: Renderable + ?Sized> {
    /// Data buffer itself
    data: &'a [u8],
    /// Offset in to the data buffer for the first byte
    pub byte_offset: u32,
    /// Length of data used in the buffer
    pub byte_length: u32,
    /// The client bound to data[byte_offset] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferView`s using
    /// it - which then have shared references to the data - but the
    /// client is created afterwards
    rc_client: RefCell<R::Buffer>,
}

//ip BufferData
impl<'a, R: Renderable> BufferData<'a, R> {
    //fp new
    /// Create a new `BufferData` given a buffer, offset and length; if the
    /// length is zero then the whole of the data buffer post offset
    /// is used
    ///
    /// If offset and length are both zero, then all the data is used
    ///
    /// This function can be invoked prior to the OpenGL context being
    /// created; this performs no OpenGL calls
    pub fn new(data: &'a dyn ByteBuffer, byte_offset: u32, byte_length: u32) -> Self {
        let byte_length = {
            if byte_length == 0 {
                (data.byte_length() as u32) - byte_offset
            } else {
                byte_length
            }
        };
        let rc_client = RefCell::new(R::Buffer::default());
        let data = data.borrow_bytes();
        Self {
            data,
            byte_offset,
            byte_length,
            rc_client,
        }
    }

    //mp create_client
    /// Replace the client data with one of this data
    pub fn create_client(&self, render_context: &mut R::Context) {
        self.rc_client.borrow_mut().create(self, render_context);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::Buffer> {
        self.rc_client.borrow()
    }

    //mp as_ptr
    /// Get a const u8 ptr to the data itself
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { self.data.as_ptr().add(self.byte_offset as usize) }
    }

    //zz All done
}

//ip Display for BufferData
impl<'a, R: Renderable + ?Sized> std::fmt::Display for BufferData<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let data_ptr = self.data.as_ptr();
        write!(
            f,
            "BufferData[{:?}+{}#{}]:GL({})",
            data_ptr,
            self.byte_offset,
            self.byte_length,
            self.rc_client.borrow()
        )
    }
}

//ip DefaultIndentedDisplay for BufferData
impl<'a, R: Renderable + ?Sized> indent_display::DefaultIndentedDisplay for BufferData<'a, R> {}
