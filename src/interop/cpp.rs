#[test] fn layout() -> Result<(), Box<dyn std::error::Error>> {
    use crate::*;

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use core::mem::*;

    let mut code_path = std::env::current_exe().expect("current_exe()");
    code_path.pop();
    let out_dir = code_path.clone();

    code_path.push("interop-layout-test.cpp");
    let code_path : &Path = &code_path;

    let bin_path  = "interop-layout-test.o";
    let mut code = std::io::BufWriter::new(File::create(code_path).expect("couldn't create code_path"));

    writeln!(code, "#define WIN32_LEAN_AND_MEAN")?;
    writeln!(code, "#include <windows.h>")?;
    writeln!(code, "#include <xinput.h>")?; // XINPUT_*
    writeln!(code, "#include <cstddef>")?; // offsetof

    writeln!(code)?;
    writeln!(code)?;
    writeln!(code)?;

    macro_rules! constants {
        ($(
            $cpp_ident:expr
            =>
            $rust:expr
        ),*$(,)?) => {
            [$(
                (stringify!($cpp_ident), stringify!($rust), format!("{}", $rust)),
            )*]
        };
    }

    use crate as xinput;
    #[allow(deprecated)] for (cpp_name, rust_name, rust_value) in constants![
        XUSER_INDEX_ANY                     => xuser::INDEX_ANY,
        XUSER_MAX_COUNT                     => xuser::MAX_COUNT,

        BATTERY_DEVTYPE_GAMEPAD             => BatteryDevType::Gamepad              .into_inner(),
        BATTERY_DEVTYPE_HEADSET             => BatteryDevType::Headset              .into_inner(),

        BATTERY_LEVEL_EMPTY                 => BatteryLevel::Empty                  .into_inner(),
        BATTERY_LEVEL_LOW                   => BatteryLevel::Low                    .into_inner(),
        BATTERY_LEVEL_MEDIUM                => BatteryLevel::Medium                 .into_inner(),
        BATTERY_LEVEL_FULL                  => BatteryLevel::Full                   .into_inner(),

        BATTERY_TYPE_DISCONNECTED           => BatteryType::Disconnected            .into_inner(),
        BATTERY_TYPE_WIRED                  => BatteryType::Wired                   .into_inner(),
        BATTERY_TYPE_ALKALINE               => BatteryType::Alkaline                .into_inner(),
        BATTERY_TYPE_NIMH                   => BatteryType::NiMH                    .into_inner(),
        BATTERY_TYPE_UNKNOWN                => BatteryType::Unknown                 .into_inner(),

        XINPUT_DEVSUBTYPE_UNKNOWN           => xinput::DevSubType::Unknown          .into_inner(),
        XINPUT_DEVSUBTYPE_GAMEPAD           => xinput::DevSubType::Gamepad          .into_inner(),
        XINPUT_DEVSUBTYPE_WHEEL             => xinput::DevSubType::Wheel            .into_inner(),
        XINPUT_DEVSUBTYPE_ARCADE_STICK      => xinput::DevSubType::ArcadeStick      .into_inner(),
        XINPUT_DEVSUBTYPE_FLIGHT_STICK      => xinput::DevSubType::FlightStick      .into_inner(),
        XINPUT_DEVSUBTYPE_DANCE_PAD         => xinput::DevSubType::DancePad         .into_inner(),
        XINPUT_DEVSUBTYPE_GUITAR            => xinput::DevSubType::Guitar           .into_inner(),
        XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE  => xinput::DevSubType::GuitarAlternate  .into_inner(),
        XINPUT_DEVSUBTYPE_DRUM_KIT          => xinput::DevSubType::DrumKit          .into_inner(),
        XINPUT_DEVSUBTYPE_GUITAR_BASS       => xinput::DevSubType::GuitarBass       .into_inner(),
        XINPUT_DEVSUBTYPE_ARCADE_PAD        => xinput::DevSubType::ArcadePad        .into_inner(),

        XINPUT_DEVTYPE_GAMEPAD              => xinput::DevType::Gamepad             .into_inner(),

        VK_PAD_A                            => VK::PadA                             .into_inner(),
        VK_PAD_B                            => VK::PadB                             .into_inner(),
        VK_PAD_X                            => VK::PadX                             .into_inner(),
        VK_PAD_Y                            => VK::PadY                             .into_inner(),
        VK_PAD_RSHOULDER                    => VK::PadRShoulder                     .into_inner(),
        VK_PAD_LSHOULDER                    => VK::PadLShoulder                     .into_inner(),
        VK_PAD_LTRIGGER                     => VK::PadLTrigger                      .into_inner(),
        VK_PAD_RTRIGGER                     => VK::PadRTrigger                      .into_inner(),
        VK_PAD_DPAD_UP                      => VK::PadDPadUp                        .into_inner(),
        VK_PAD_DPAD_DOWN                    => VK::PadDPadDown                      .into_inner(),
        VK_PAD_DPAD_LEFT                    => VK::PadDPadLeft                      .into_inner(),
        VK_PAD_DPAD_RIGHT                   => VK::PadDPadRight                     .into_inner(),
        VK_PAD_START                        => VK::PadStart                         .into_inner(),
        VK_PAD_BACK                         => VK::PadBack                          .into_inner(),
        VK_PAD_LTHUMB_PRESS                 => VK::PadLThumbPress                   .into_inner(),
        VK_PAD_RTHUMB_PRESS                 => VK::PadRThumbPress                   .into_inner(),
        VK_PAD_LTHUMB_UP                    => VK::PadLThumbUp                      .into_inner(),
        VK_PAD_LTHUMB_DOWN                  => VK::PadLThumbDown                    .into_inner(),
        VK_PAD_LTHUMB_RIGHT                 => VK::PadLThumbRight                   .into_inner(),
        VK_PAD_LTHUMB_LEFT                  => VK::PadLThumbLeft                    .into_inner(),
        VK_PAD_LTHUMB_UPLEFT                => VK::PadLThumbUpLeft                  .into_inner(),
        VK_PAD_LTHUMB_UPRIGHT               => VK::PadLThumbUpRight                 .into_inner(),
        VK_PAD_LTHUMB_DOWNRIGHT             => VK::PadLThumbDownRight               .into_inner(),
        VK_PAD_LTHUMB_DOWNLEFT              => VK::PadLThumbDownLeft                .into_inner(),
        VK_PAD_RTHUMB_UP                    => VK::PadRThumbUp                      .into_inner(),
        VK_PAD_RTHUMB_DOWN                  => VK::PadRThumbDown                    .into_inner(),
        VK_PAD_RTHUMB_RIGHT                 => VK::PadRThumbRight                   .into_inner(),
        VK_PAD_RTHUMB_LEFT                  => VK::PadRThumbLeft                    .into_inner(),
        VK_PAD_RTHUMB_UPLEFT                => VK::PadRThumbUpLeft                  .into_inner(),
        VK_PAD_RTHUMB_UPRIGHT               => VK::PadRThumbUpRight                 .into_inner(),
        VK_PAD_RTHUMB_DOWNRIGHT             => VK::PadRThumbDownRight               .into_inner(),
        VK_PAD_RTHUMB_DOWNLEFT              => VK::PadRThumbDownLeft                .into_inner(),

        XINPUT_GAMEPAD_DPAD_UP              => xinput::Buttons::DPadUp              .into_inner(),
        XINPUT_GAMEPAD_DPAD_DOWN            => xinput::Buttons::DPadDown            .into_inner(),
        XINPUT_GAMEPAD_DPAD_LEFT            => xinput::Buttons::DPadLeft            .into_inner(),
        XINPUT_GAMEPAD_DPAD_RIGHT           => xinput::Buttons::DPadRight           .into_inner(),
        XINPUT_GAMEPAD_START                => xinput::Buttons::Start               .into_inner(),
        XINPUT_GAMEPAD_BACK                 => xinput::Buttons::Back                .into_inner(),
        XINPUT_GAMEPAD_LEFT_THUMB           => xinput::Buttons::LeftThumb           .into_inner(),
        XINPUT_GAMEPAD_RIGHT_THUMB          => xinput::Buttons::RightThumb          .into_inner(),
        XINPUT_GAMEPAD_LEFT_SHOULDER        => xinput::Buttons::LeftShoulder        .into_inner(),
        XINPUT_GAMEPAD_RIGHT_SHOULDER       => xinput::Buttons::RightShoulder       .into_inner(),
        XINPUT_GAMEPAD_A                    => xinput::Buttons::A                   .into_inner(),
        XINPUT_GAMEPAD_B                    => xinput::Buttons::B                   .into_inner(),
        XINPUT_GAMEPAD_X                    => xinput::Buttons::X                   .into_inner(),
        XINPUT_GAMEPAD_Y                    => xinput::Buttons::Y                   .into_inner(),

        XINPUT_CAPS_VOICE_SUPPORTED         => xinput::Caps::VoiceSupported         .into_inner(),
        XINPUT_CAPS_FFB_SUPPORTED           => xinput::Caps::FfbSupported           .into_inner(),
        XINPUT_CAPS_WIRELESS                => xinput::Caps::Wireless               .into_inner(),
        XINPUT_CAPS_PMD_SUPPORTED           => xinput::Caps::PmdSupported           .into_inner(),
        XINPUT_CAPS_NO_NAVIGATION           => xinput::Caps::NoNavigation           .into_inner(),

        XINPUT_FLAG_GAMEPAD                 => xinput::Flag::Gamepad                .into_inner(),

        XINPUT_KEYSTROKE_KEYDOWN            => xinput::Keystroke::KeyDown           .into_inner(),
        XINPUT_KEYSTROKE_KEYUP              => xinput::Keystroke::KeyUp             .into_inner(),
        XINPUT_KEYSTROKE_REPEAT             => xinput::Keystroke::Repeat            .into_inner(),
    ] {
        writeln!(code, "static_assert({cpp_name} == {rust_value}, \"{cpp_name} != {rust_name}\");")?;
    }

    macro_rules! structs {
        ($(
            $rust_struct:ident => $cpp_struct:ident {
                $( $rust_field:ident => $cpp_field:ident ),* $(,)?
            }
        )*) => {{$(
            let rust_align = align_of::<$rust_struct>();
            let rust_size  =  size_of::<$rust_struct>();
            let rust_struct_name  = stringify!($rust_struct);
            let cpp_struct_name   = stringify!($cpp_struct);

            writeln!(code)?;
            writeln!(code)?;
            writeln!(code)?;
            writeln!(code, "static_assert({rust_align: >2} == alignof({cpp_struct_name}), \"alignof(xinput::{rust_struct_name}) != alignof({cpp_struct_name})\");")?;
            writeln!(code, "static_assert({rust_size: >2} ==  sizeof({cpp_struct_name}),  \"sizeof(xinput::{rust_struct_name}) != sizeof({cpp_struct_name})\");")?;

            let _rust_struct = <$rust_struct as bytemuck::Zeroable>::zeroed();
            $({
                let rust_align      = align_of_val(&_rust_struct.$rust_field);
                let rust_size       =  size_of_val(&_rust_struct.$rust_field);
                let rust_offset     = { let base : *const _ = &_rust_struct; let field : *const _ = &_rust_struct.$rust_field; (field as usize) - (base as usize) };
                let rust_field_name = stringify!($rust_field);
                let cpp_field_name  = stringify!($cpp_field);

                writeln!(code)?;
                writeln!(code, "static_assert({rust_align: >2} == alignof(decltype({cpp_struct_name}{{}}.{cpp_field_name})), \"alignof(xinput::{rust_struct_name}::{rust_field_name}) != alignof({cpp_struct_name}::{cpp_field_name})\");")?;
                writeln!(code, "static_assert({rust_size: >2} ==  sizeof(decltype({cpp_struct_name}{{}}.{cpp_field_name})),  \"sizeof(xinput::{rust_struct_name}::{rust_field_name}) != sizeof({cpp_struct_name}::{cpp_field_name})\");")?;
                writeln!(code, "static_assert({rust_offset: >2} == offsetof({cpp_struct_name}, {cpp_field_name}), \"offsetof(xinput::{rust_struct_name}, {rust_field_name}) != offsetof({cpp_struct_name}, {cpp_field_name})\");")?;
            })*
        )*}};
    }

    structs! {
        BatteryInformation => XINPUT_BATTERY_INFORMATION {
            battery_type        => BatteryType,
            battery_level       => BatteryLevel,
        }

        Capabilities => XINPUT_CAPABILITIES {
            ty                  => Type,
            sub_type            => SubType,
            flags               => Flags,
            gamepad             => Gamepad,
            vibration           => Vibration,
        }

        Gamepad => XINPUT_GAMEPAD {
            buttons             => wButtons,
            left_trigger        => bLeftTrigger,
            right_trigger       => bRightTrigger,
            left_thumb_x        => sThumbLX,
            left_thumb_y        => sThumbLY,
            right_thumb_x       => sThumbRX,
            right_thumb_y       => sThumbRY,
        }

        DSoundAudioDeviceGuid => GUID {
            // fields not individually defined
        }

        Keystroke => XINPUT_KEYSTROKE {
            virtual_key         => VirtualKey,
            unicode             => Unicode,
            flags               => Flags,
            user_index          => UserIndex,
            hid_code            => HidCode,
        }

        State => XINPUT_STATE {
            packet_number       => dwPacketNumber,
            gamepad             => Gamepad,
        }

        Vibration => XINPUT_VIBRATION {
            left_motor_speed    => wLeftMotorSpeed,
            right_motor_speed   => wRightMotorSpeed,
        }
    }

    drop(code);

    for target in ["x86_64-pc-windows-msvc", "i686-pc-windows-msvc"] {
        cc::Build::new()
            .file(code_path)
            .out_dir(&out_dir)
            .opt_level(0)
            .target(target)
            .host(target)
            .cpp(true)
            .emit_rerun_if_env_changed(false)
            //.std("c++11") // alignof, static_assert
            .try_compile(bin_path)?;
    }

    Ok(())
}
