use alloc::{boxed::Box, vec};

use async_trait::async_trait;
use jvm::{ClassInstanceRef, JavaError, JavaValue, Jvm, Result as JvmResult};
use jvm_class_proto::{JavaMethodProto, MethodBody};
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use wie_jvm_support::{WieJavaClassProto, WieJvmContext};

use crate::classes::org::kwis::msp::media::{BaseClip, Clip, PlayListener};

// class org.kwis.msp.media.Player
pub struct Player;

impl Player {
    pub fn as_proto() -> WieJavaClassProto {
        WieJavaClassProto {
            name: "org/kwis/msp/media/Player",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "pause",
                    "(Lorg/kwis/msp/media/BaseClip;)Z",
                    Self::pause,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "stop",
                    "(Lorg/kwis/msp/media/BaseClip;)Z",
                    Self::stop,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "resume",
                    "(Lorg/kwis/msp/media/BaseClip;)Z",
                    Self::resume,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "play",
                    "(Lorg/kwis/msp/media/BaseClip;Z)Z",
                    Self::play,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "record",
                    "(Lorg/kwis/msp/media/BaseClip;)Z",
                    Self::record,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "play",
                    "(Lorg/kwis/msp/media/Clip;Z)Z",
                    Self::play_clip,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "stop",
                    "(Lorg/kwis/msp/media/Clip;)Z",
                    Self::stop_clip,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn pause(_: &Jvm, _: &mut WieJvmContext, clip: ClassInstanceRef<BaseClip>) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::pause({clip:?})");

        Ok(false)
    }

    async fn stop(_: &Jvm, _: &mut WieJvmContext, clip: ClassInstanceRef<BaseClip>) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::stop({clip:?})");

        Ok(false)
    }

    async fn resume(_: &Jvm, _: &mut WieJvmContext, clip: ClassInstanceRef<BaseClip>) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::resume({clip:?})");

        Ok(false)
    }

    async fn play(_: &Jvm, _: &mut WieJvmContext, clip: ClassInstanceRef<BaseClip>, repeat: bool) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::play({clip:?}, {repeat})");

        Ok(false)
    }

    async fn record(_: &Jvm, _: &mut WieJvmContext, clip: ClassInstanceRef<BaseClip>) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::record({clip:?})");

        Ok(false)
    }

    async fn play_clip(jvm: &Jvm, context: &mut WieJvmContext, clip: ClassInstanceRef<Clip>, repeat: bool) -> JvmResult<bool> {
        tracing::debug!("org.kwis.msp.media.Player::play({clip:?}, {repeat})");

        let player = Clip::player(jvm, &clip).await?;

        if player.is_null() {
            return Ok(false);
        }

        let _: () = jvm.invoke_virtual(&player, "net/wie/SmafPlayer", "start", "(Z)V", (repeat,)).await?;

        Clip::notify_listener(jvm, &clip, PlayListener::START, 0).await?;

        // WIPI titles that wait on PlayListener.END_OF_DATA after splash audio.
        // Fire it after the sequence duration (or a short fallback) when not looping.
        if !repeat {
            let audio_handle: i32 = jvm.get_field(&player, "audioHandle", "I").await?;
            let duration_ms = context
                .system()
                .audio()
                .duration(audio_handle as u32)
                .unwrap_or(0)
                .max(500);

            context.spawn(
                jvm,
                Box::new(EndOfDataNotifier {
                    clip: clip.clone(),
                    duration_ms,
                }),
            )?;
        }

        Ok(true)
    }

    async fn stop_clip(jvm: &Jvm, _: &mut WieJvmContext, clip: ClassInstanceRef<Clip>) -> JvmResult<bool> {
        tracing::debug!("org.kwis.msp.media.Player::stop({clip:?})");

        let player = Clip::player(jvm, &clip).await?;

        if !player.is_null() {
            let _: () = jvm.invoke_virtual(&player, "javax/microedition/media/Player", "stop", "()V", ()).await?;
            Clip::notify_listener(jvm, &clip, PlayListener::STOP, 0).await?;

            return Ok(true);
        }

        Ok(false)
    }
}

struct EndOfDataNotifier {
    clip: ClassInstanceRef<Clip>,
    duration_ms: u64,
}

#[async_trait]
impl MethodBody<JavaError, WieJvmContext> for EndOfDataNotifier {
    async fn call(&self, jvm: &Jvm, context: &mut WieJvmContext, _args: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
        jvm.attach_thread(None).await?;
        context.system().sleep(self.duration_ms).await;
        Clip::notify_listener(jvm, &self.clip, PlayListener::END_OF_DATA, 0).await?;
        Clip::notify_listener(jvm, &self.clip, PlayListener::STOP, 0).await?;
        Ok(JavaValue::Void)
    }
}

#[cfg(test)]
mod test {
    use alloc::boxed::Box;

    use jvm::{ClassInstanceRef, runtime::JavaLangString};
    use rustjava_runtime::classes::java::lang::String;
    use test_utils::run_jvm_test;
    use wie_util::Result;

    use crate::{
        classes::org::kwis::msp::media::{BaseClip, Clip},
        get_protos,
    };

    #[test]
    fn test_base_clip_player_methods_are_callable() -> Result<()> {
        run_jvm_test(Box::new([wie_midp::get_protos().into(), get_protos().into()]), |jvm| async move {
            let clip: ClassInstanceRef<BaseClip> = jvm.new_class("org/kwis/msp/media/BaseClip", "()V", ()).await?.into();

            let _: bool = jvm
                .invoke_static("org/kwis/msp/media/Player", "pause", "(Lorg/kwis/msp/media/BaseClip;)Z", (clip.clone(),))
                .await?;
            let _: bool = jvm
                .invoke_static("org/kwis/msp/media/Player", "stop", "(Lorg/kwis/msp/media/BaseClip;)Z", (clip.clone(),))
                .await?;
            let _: bool = jvm
                .invoke_static("org/kwis/msp/media/Player", "resume", "(Lorg/kwis/msp/media/BaseClip;)Z", (clip.clone(),))
                .await?;
            let _: bool = jvm
                .invoke_static(
                    "org/kwis/msp/media/Player",
                    "play",
                    "(Lorg/kwis/msp/media/BaseClip;Z)Z",
                    (clip.clone(), false),
                )
                .await?;
            let _: bool = jvm
                .invoke_static("org/kwis/msp/media/Player", "record", "(Lorg/kwis/msp/media/BaseClip;)Z", (clip,))
                .await?;

            Ok(())
        })
    }

    #[test]
    fn test_clip_play_and_stop() -> Result<()> {
        run_jvm_test(Box::new([wie_midp::get_protos().into(), get_protos().into()]), |jvm| async move {
            let r#type = JavaLangString::from_rust_string(&jvm, "audio/mmf").await?;
            let mut data = jvm.instantiate_array("B", 0).await?;
            jvm.store_array(&mut data, 0, [] as [i8; 0]).await?;

            let clip: ClassInstanceRef<Clip> = jvm
                .new_class("org/kwis/msp/media/Clip", "(Ljava/lang/String;[B)V", (r#type, data))
                .await?
                .into();

            let _: bool = jvm
                .invoke_static("org/kwis/msp/media/Player", "play", "(Lorg/kwis/msp/media/Clip;Z)Z", (clip.clone(), true))
                .await?;
            let _: bool = jvm
                .invoke_static("org/kwis/msp/media/Player", "stop", "(Lorg/kwis/msp/media/Clip;)Z", (clip,))
                .await?;

            Ok(())
        })
    }
}
