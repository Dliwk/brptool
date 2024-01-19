use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum BaMessage {
    SessionReset = 0,
    SessionCommands = 1,
    SessionDynamicsCorrection = 2,
    Null = 3,
    RequestRemotePlayer = 4,
    AttachRemotePlayer = 5,
    DetachRemotePlayer = 6,
    RemotePlayerInputCommands = 7,
    RemoveRemotePlayer = 8,
    PartyRoster = 9,
    Chat = 10,
    PartyMemberJoined = 11,
    PartyMemberLeft = 12,
    Multipart = 13,
    MultipartEnd = 14,
    ClientPlayerProfiles = 15,
    AttachRemotePlayer2 = 16,
    HostInfo = 17,
    ClientInfo = 18,
    KickVote = 19,
    JMessage = 20,
    ClientPlayerProfilesJson = 21,
}

/// Command values sent across the wire in netplay.
/// Must remain consistent across versions!
#[derive(Debug, FromPrimitive)]
pub enum SessionCommand {
    BaseTimeStep,
    StepSceneGraph,
    AddSceneGraph,
    RemoveSceneGraph,
    AddNode,
    NodeOnCreate,
    SetForegroundSceneGraph,
    RemoveNode,
    AddMaterial,
    RemoveMaterial,
    AddMaterialComponent,
    AddTexture,
    RemoveTexture,
    AddModel,
    RemoveModel,
    AddSound,
    RemoveSound,
    AddCollideModel,
    RemoveCollideModel,
    ConnectNodeAttribute,
    NodeMessage,
    SetNodeAttrFloat,
    SetNodeAttrInt32,
    SetNodeAttrBool,
    SetNodeAttrFloats,
    SetNodeAttrInt32s,
    SetNodeAttrString,
    SetNodeAttrNode,
    SetNodeAttrNodeNull,
    SetNodeAttrNodes,
    SetNodeAttrPlayer,
    SetNodeAttrPlayerNull,
    SetNodeAttrMaterials,
    SetNodeAttrTexture,
    SetNodeAttrTextureNull,
    SetNodeAttrTextures,
    SetNodeAttrSound,
    SetNodeAttrSoundNull,
    SetNodeAttrSounds,
    SetNodeAttrModel,
    SetNodeAttrModelNull,
    SetNodeAttrModels,
    SetNodeAttrCollideModel,
    SetNodeAttrCollideModelNull,
    SetNodeAttrCollideModels,
    PlaySoundAtPosition,
    PlaySound,
    EmitBGDynamics,
    EndOfFile,
    DynamicsCorrection,
    ScreenMessageBottom,
    ScreenMessageTop,
    AddData,
    RemoveData,
}