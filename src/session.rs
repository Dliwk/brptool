use num::FromPrimitive;

use crate::consts::{BaMessage, SessionCommand};

fn handle_command(command: &[u8]) {
    let cmd_type = SessionCommand::from_u8(command[0]).unwrap();
    match cmd_type {
        SessionCommand::BaseTimeStep => {}
        SessionCommand::StepSceneGraph => {}
        SessionCommand::AddSceneGraph => {}
        SessionCommand::RemoveSceneGraph => {}
        SessionCommand::AddNode => {}
        SessionCommand::NodeOnCreate => {}
        SessionCommand::SetForegroundSceneGraph => {}
        SessionCommand::RemoveNode => {}
        SessionCommand::AddMaterial => {}
        SessionCommand::RemoveMaterial => {}
        SessionCommand::AddMaterialComponent => {}
        SessionCommand::AddTexture => {}
        SessionCommand::RemoveTexture => {}
        SessionCommand::AddModel => {}
        SessionCommand::RemoveModel => {}
        SessionCommand::AddSound => {}
        SessionCommand::RemoveSound => {}
        SessionCommand::AddCollideModel => {}
        SessionCommand::RemoveCollideModel => {}
        SessionCommand::ConnectNodeAttribute => {}
        SessionCommand::NodeMessage => {}
        SessionCommand::SetNodeAttrFloat => {}
        SessionCommand::SetNodeAttrInt32 => {}
        SessionCommand::SetNodeAttrBool => {}
        SessionCommand::SetNodeAttrFloats => {}
        SessionCommand::SetNodeAttrInt32s => {}
        SessionCommand::SetNodeAttrString => {}
        SessionCommand::SetNodeAttrNode => {}
        SessionCommand::SetNodeAttrNodeNull => {}
        SessionCommand::SetNodeAttrNodes => {}
        SessionCommand::SetNodeAttrPlayer => {}
        SessionCommand::SetNodeAttrPlayerNull => {}
        SessionCommand::SetNodeAttrMaterials => {}
        SessionCommand::SetNodeAttrTexture => {}
        SessionCommand::SetNodeAttrTextureNull => {}
        SessionCommand::SetNodeAttrTextures => {}
        SessionCommand::SetNodeAttrSound => {}
        SessionCommand::SetNodeAttrSoundNull => {}
        SessionCommand::SetNodeAttrSounds => {}
        SessionCommand::SetNodeAttrModel => {}
        SessionCommand::SetNodeAttrModelNull => {}
        SessionCommand::SetNodeAttrModels => {}
        SessionCommand::SetNodeAttrCollideModel => {}
        SessionCommand::SetNodeAttrCollideModelNull => {}
        SessionCommand::SetNodeAttrCollideModels => {}
        SessionCommand::PlaySoundAtPosition => {}
        SessionCommand::PlaySound => {}
        SessionCommand::EmitBGDynamics => {}
        SessionCommand::EndOfFile => {}
        SessionCommand::DynamicsCorrection => {}
        SessionCommand::ScreenMessageBottom => {}
        SessionCommand::ScreenMessageTop => {}
        SessionCommand::AddData => {}
        SessionCommand::RemoveData => {}
    }
}

fn handle_commands(buffer: &[u8]) {
    let mut offset = 1;
    loop {
        let size: usize = u16::from_le_bytes({
            let buf: [u8; 2]
                = buffer[offset..][..2].try_into().unwrap();
            buf
        }) as usize;
        if offset + size > buffer.len() {
            panic!("invalid state message");
        }
        let sub_buffer = &buffer[offset + 2..][..size];

        handle_command(sub_buffer);

        offset += 2 + size;

        if offset == buffer.len() {
            break;
        }
    }
}

pub fn handle_session_message(buffer: &[u8]) {
    let msg_type = BaMessage::from_u8(buffer[0])
        .expect("got incorrect message type");
    dbg!(&msg_type);
    match msg_type {
        BaMessage::SessionReset => {}
        BaMessage::SessionCommands => {
            handle_commands(buffer);
        }
        BaMessage::SessionDynamicsCorrection => {}
        BaMessage::Null => {}
        BaMessage::RequestRemotePlayer => {}
        BaMessage::AttachRemotePlayer => {}
        BaMessage::DetachRemotePlayer => {}
        BaMessage::RemotePlayerInputCommands => {}
        BaMessage::RemoveRemotePlayer => {}
        BaMessage::PartyRoster => {}
        BaMessage::Chat => {}
        BaMessage::PartyMemberJoined => {}
        BaMessage::PartyMemberLeft => {}
        BaMessage::Multipart => {}
        BaMessage::MultipartEnd => {}
        BaMessage::ClientPlayerProfiles => {}
        BaMessage::AttachRemotePlayer2 => {}
        BaMessage::HostInfo => {}
        BaMessage::ClientInfo => {}
        BaMessage::KickVote => {}
        BaMessage::JMessage => {}
        BaMessage::ClientPlayerProfilesJson => {}
    }
}
