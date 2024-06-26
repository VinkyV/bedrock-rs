use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use bedrock_core::LE;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

use crate::types::pack_info_behavior::BehaviorPackInfoType;
use crate::types::pack_info_resource::ResourcePackInfoType;
use crate::types::pack_url::PackURL;

#[derive(Debug, Clone)]
pub struct ResourcePacksInfoPacket {
    pub resource_pack_required: bool,
    pub has_addon_packs: bool,
    pub has_scripts: bool,
    pub force_server_packs_enabled: bool,
    pub behavior_packs: Vec<BehaviorPackInfoType>,
    pub resource_packs: Vec<ResourcePackInfoType>,
    pub cdn_urls: Vec<PackURL>,
}

impl ProtoCodec for ResourcePacksInfoPacket {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        // Serialize resource_pack_required as a bool
        match self.resource_pack_required.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Serialize has_addon_packs as a bool
        match self.has_addon_packs.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Serialize has_scripts as a bool
        match self.has_scripts.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Serialize force_server_packs_enabled as a bool
        match self.force_server_packs_enabled.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let len = match self.behavior_packs.len().try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        // Write length of behavior packs as an u16le
        match LE::<u16>::new(len).proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Write every behavior pack
        for behavior_pack in &self.behavior_packs {
            match behavior_pack.proto_serialize(stream) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        let len = match self.resource_packs.len().try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        // Write length of resource packs as an u16le
        match LE::<u16>::new(len).proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Write every resource pack
        for behavior_pack in &self.resource_packs {
            match behavior_pack.proto_serialize(stream) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        // Write all cdn urls
        match self.cdn_urls.proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        todo!()
    }
}
