// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: block.capnp


pub mod body {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader: reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_parent_id(self) -> Result<data::Reader<'a>> {
      self.reader.get_pointer_field(0).get_data(::std::ptr::null(), 0)
    }
    pub fn has_parent_id(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_timestamp(self) -> u64 {
      self.reader.get_data_field::<u64>(0)
    }
    #[inline]
    pub fn get_ops(self) -> Result<struct_list::Reader<'a,::object_capnp::object::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1))
    }
    pub fn has_ops(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_comment(self) -> Result<text::Reader<'a>> {
      self.reader.get_pointer_field(2).get_text(::std::ptr::null(), 0)
    }
    pub fn has_comment(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder: builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_parent_id(self) -> Result<data::Builder<'a>> {
      self.builder.get_pointer_field(0).get_data(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_parent_id(&mut self, value: data::Reader)  {
      self.builder.get_pointer_field(0).set_data(value);
    }
    #[inline]
    pub fn init_parent_id(self, size: u32) -> data::Builder<'a> {
      self.builder.get_pointer_field(0).init_data(size)
    }
    pub fn has_parent_id(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_timestamp(self) -> u64 {
      self.builder.get_data_field::<u64>(0)
    }
    #[inline]
    pub fn set_timestamp(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(0, value);
    }
    #[inline]
    pub fn get_ops(self) -> Result<struct_list::Builder<'a,::object_capnp::object::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1))
    }
    #[inline]
    pub fn set_ops(&mut self, value: struct_list::Reader<'a,::object_capnp::object::Owned>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(1), value)
    }
    #[inline]
    pub fn init_ops(self, size: u32) -> struct_list::Builder<'a,::object_capnp::object::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), size)
    }
    pub fn has_ops(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_comment(self) -> Result<text::Builder<'a>> {
      self.builder.get_pointer_field(2).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_comment(&mut self, value: text::Reader)  {
      self.builder.get_pointer_field(2).set_text(value);
    }
    #[inline]
    pub fn init_comment(self, size: u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(2).init_text(size)
    }
    pub fn has_comment(&self) -> bool {
      !self.builder.get_pointer_field(2).is_null()
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 3 };
    pub const TYPE_ID: u64 = 0xc4aac63e83296aab;
  }
}

pub mod witness {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader: reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_signatures(self) -> Result<struct_list::Reader<'a,::signature_capnp::signature::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0))
    }
    pub fn has_signatures(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder: builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_signatures(self) -> Result<struct_list::Builder<'a,::signature_capnp::signature::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0))
    }
    #[inline]
    pub fn set_signatures(&mut self, value: struct_list::Reader<'a,::signature_capnp::signature::Owned>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value)
    }
    #[inline]
    pub fn init_signatures(self, size: u32) -> struct_list::Builder<'a,::signature_capnp::signature::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), size)
    }
    pub fn has_signatures(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 0, pointers: 1 };
    pub const TYPE_ID: u64 = 0xb9da30c18b074b8c;
  }
}

pub mod block {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader: reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_body(self) -> Result<::block_capnp::body::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0))
    }
    pub fn has_body(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_witness(self) -> Result<::block_capnp::witness::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1))
    }
    pub fn has_witness(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder: builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_body(self) -> Result<::block_capnp::body::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0))
    }
    #[inline]
    pub fn set_body<'b>(&mut self, value: ::block_capnp::body::Reader<'b>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value)
    }
    #[inline]
    pub fn init_body(self, ) -> ::block_capnp::body::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_body(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_witness(self) -> Result<::block_capnp::witness::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1))
    }
    #[inline]
    pub fn set_witness<'b>(&mut self, value: ::block_capnp::witness::Reader<'b>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(1), value)
    }
    #[inline]
    pub fn init_witness(self, ) -> ::block_capnp::witness::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), 0)
    }
    pub fn has_witness(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_body(&self) -> ::block_capnp::body::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
    pub fn get_witness(&self) -> ::block_capnp::witness::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(1))
    }
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 0, pointers: 2 };
    pub const TYPE_ID: u64 = 0xf909a26c24f13089;
  }
}
