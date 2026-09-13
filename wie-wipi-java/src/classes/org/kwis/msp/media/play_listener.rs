use alloc::vec;

use jvm::{Jvm, Result as JvmResult};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use wie_jvm_support::{WieJavaClassProto, WieJvmContext};

// interface org.kwis.msp.media.PlayListener
pub struct PlayListener;

impl PlayListener {
    pub const ERROR: i32 = -1;
    pub const END_OF_DATA: i32 = 1;
    pub const START: i32 = 2;
    pub const STOP: i32 = 3;
    pub const PAUSE: i32 = 4;
    pub const RESUME: i32 = 5;
    pub const RECORD: i32 = 6;
    pub const FULL_OF_DATA: i32 = 7;

    pub fn as_proto() -> WieJavaClassProto {
        WieJavaClassProto {
            name: "org/kwis/msp/media/PlayListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::cl_init, MethodAccessFlags::STATIC),
                JavaMethodProto::new_abstract(
                    "playUpdate",
                    "(Lorg/kwis/msp/media/Clip;II)V",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("ERROR", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "END_OF_DATA",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("START", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STOP", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PAUSE", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("RESUME", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("RECORD", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "FULL_OF_DATA",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn cl_init(jvm: &Jvm, _context: &mut WieJvmContext) -> JvmResult<()> {
        tracing::debug!("org.kwis.msp.media.PlayListener::<clinit>");

        jvm.put_static_field("org/kwis/msp/media/PlayListener", "ERROR", "I", Self::ERROR).await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "END_OF_DATA", "I", Self::END_OF_DATA)
            .await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "START", "I", Self::START).await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "STOP", "I", Self::STOP).await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "PAUSE", "I", Self::PAUSE).await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "RESUME", "I", Self::RESUME).await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "RECORD", "I", Self::RECORD).await?;
        jvm.put_static_field("org/kwis/msp/media/PlayListener", "FULL_OF_DATA", "I", Self::FULL_OF_DATA)
            .await?;

        Ok(())
    }
}
