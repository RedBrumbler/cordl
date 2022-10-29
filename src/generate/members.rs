use super::{context::CppCommentedString, writer::Writable};
use std::io::Write;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CppMember {
    Field(CppField),
    Method(CppMethod),
    Property(CppProperty),
    Comment(CppCommentedString),
    // TODO: Or a nested type
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CppMethodData {
    pub estimated_size: usize,
    pub addrs: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CppField {
    pub name: String,
    pub ty: String,
    pub offset: u32,
    pub instance: bool,
    pub readonly: bool,
    pub classof_call: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CppParam {
    pub name: String,
    pub ty: String,
    // TODO: Use bitflags to indicate these attributes
    // May hold:
    // const
    // May hold one of:
    // *
    // &
    // &&
    pub modifiers: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CppMethod {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<CppParam>,
    pub instance: bool,
    // TODO: Use bitflags to indicate these attributes
    // Holds unique of:
    // const
    // override
    // noexcept
    pub suffix_modifiers: String,
    // Holds unique of:
    // constexpr
    // static
    // inline
    // explicit(...)
    // virtual
    pub prefix_modifiers: String,
    // TODO: Add all descriptions missing for the method
    pub method_data: CppMethodData,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CppProperty {
    pub name: String,
    pub ty: String,
    pub setter: Option<CppMethodData>,
    pub getter: Option<CppMethodData>,
    pub abstr: bool,
    pub instance: bool,
    pub classof_call: String,
}

impl CppField {
    pub fn make() -> CppField {
        CppField {
            name: todo!(),
            ty: todo!(),
            offset: todo!(),
            instance: todo!(),
            readonly: todo!(),
            classof_call: todo!(),
        }
    }
}

impl CppMethod {
    pub fn make() -> CppMethod {
        CppMethod {
            name: todo!(),
            return_type: todo!(),
            parameters: todo!(),
            instance: todo!(),
            suffix_modifiers: todo!(),
            prefix_modifiers: todo!(),
            method_data: todo!(),
        }
    }
}

impl CppProperty {
    pub fn make() -> CppProperty {
        CppProperty {
            name: todo!(),
            ty: todo!(),
            setter: todo!(),
            getter: todo!(),
            abstr: todo!(),
            instance: todo!(),
            classof_call: todo!(),
        }
    }
}

// Writing

impl Writable for CppField {
    fn write(&self, writer: &mut super::writer::CppWriter) -> color_eyre::Result<()> {
        writeln!(
            writer,
            "// Field: name: {}, Type Name: {}, Offset: 0x{:x}",
            self.name, self.ty, self.offset
        )?;

        if self.instance {
            writeln!(
                writer,
                "::bs_hook::InstanceField<{}, 0x{:x},{}> {};",
                self.ty, self.offset, !self.readonly, self.name
            )?;
        } else {
            writeln!(
                writer,
                "static inline ::bs_hook::StaticField<{},\"{}\",{},{}> {};",
                self.ty, self.name, !self.readonly, self.classof_call, self.name
            )?;
        }
        Ok(())
    }
}
impl Writable for CppMethod {
    fn write(&self, writer: &mut super::writer::CppWriter) -> color_eyre::Result<()> {
        writeln!(
            writer,
            "// Method: name: {}, Return Type Name: {} Parameters: {:?} Addr {:x} Size {:x}",
            self.name,
            self.return_type,
            self.parameters,
            self.method_data.addrs,
            self.method_data.estimated_size
        )?;

        Ok(())
    }
}
impl Writable for CppProperty {
    fn write(&self, writer: &mut super::writer::CppWriter) -> color_eyre::Result<()> {
        writeln!(
            writer,
            "// Property: name: {}, Type Name: {}, setter {} getter {} abstract {}",
            self.name,
            self.ty,
            self.setter.is_some(),
            self.getter.is_some(),
            self.abstr
        )?;

        // TODO:
        if self.abstr {
            return Ok(());
        }

        if self.instance {
            writeln!(
                writer,
                "::bs_hook::InstanceProperty<{},\"{}\",{},{}> {};",
                self.name,
                self.ty,
                self.getter.is_some(),
                self.setter.is_some(),
                self.name
            )?;
        } else {
            writeln!(
                writer,
                "static inline ::bs_hook::StaticProperty<{},\"{}\",{},{}, {}> {};",
                self.ty,
                self.name,
                self.getter.is_some(),
                self.setter.is_some(),
                self.classof_call,
                self.name
            )?;
        }

        Ok(())
    }
}

impl Writable for CppMember {
    fn write(&self, writer: &mut super::writer::CppWriter) -> color_eyre::Result<()> {
        match self {
            CppMember::Field(f) => f.write(writer),
            CppMember::Method(m) => m.write(writer),
            CppMember::Property(p) => p.write(writer),
            CppMember::Comment(c) => c.write(writer),
        }
    }
}
