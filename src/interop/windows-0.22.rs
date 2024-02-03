use crate::*;

struct_mapping! {
    // `windows-sys "0.27.0" ..= "0.42.0"` has many broken definitions where fields are defined as `u32`s instead of `u8`s:
    //
    // #[derive(unsafe { AsRef, AsMut, FromInto })]
    // BatteryInformation => XINPUT_BATTERY_INFORMATION { ... } // `BatteryType`, `BatteryLevel`
    //
    // #[derive(unsafe { AsRef, AsMut, FromInto })]
    // Capabilities => XINPUT_CAPABILITIES { ... } // `Type`, `SubType`

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Gamepad => XINPUT_GAMEPAD {
        buttons         => wButtons,
        left_trigger    => bLeftTrigger,
        right_trigger   => bRightTrigger,
        #[renamed] left_thumb_x    => sThumbLX,
        #[renamed] left_thumb_y    => sThumbLY,
        #[renamed] right_thumb_x   => sThumbRX,
        #[renamed] right_thumb_y   => sThumbRY,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    #[renamed] DSoundAudioDeviceGuid => GUID {
        // fields not individually defined
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Keystroke => XINPUT_KEYSTROKE {
        virtual_key     => VirtualKey,
        unicode         => Unicode,
        flags           => Flags,
        user_index      => UserIndex,
        hid_code        => HidCode,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    State => XINPUT_STATE {
        packet_number   => dwPacketNumber,
        gamepad         => Gamepad,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Vibration => XINPUT_VIBRATION {
        left_motor_speed    => wLeftMotorSpeed,
        right_motor_speed   => wRightMotorSpeed,
    }
}
