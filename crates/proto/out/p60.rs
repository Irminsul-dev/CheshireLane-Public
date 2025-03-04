// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[cmdid(60004)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60004 {
    #[prost(message, repeated, tag = "2")]
    pub request_list: ::prost::alloc::vec::Vec<MsgInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60037)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60037 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60014)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60014 {
    #[prost(uint32, required, tag = "1")]
    pub player_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGuildInfo {
    #[prost(uint32, required, tag = "1")]
    pub donate_count: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub donate_tasks: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "5")]
    pub weekly_task_flag: u32,
    #[prost(uint32, required, tag = "7")]
    pub extra_operation: u32,
    #[prost(uint32, required, tag = "3")]
    pub benefit_time: u32,
    #[prost(uint32, required, tag = "6")]
    pub extra_donate: u32,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub tech_id: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60002)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60002 {
    #[prost(uint32, required, tag = "2")]
    pub id: u32,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60020)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60020 {
    #[prost(uint32, required, tag = "1")]
    pub player_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60010)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60010 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60033)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60033 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60101)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60101 {
    #[prost(message, repeated, tag = "1")]
    pub chat_list: ::prost::alloc::vec::Vec<GuideChat>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuideChat {
    #[prost(uint32, required, tag = "3")]
    pub time: u32,
    #[prost(message, required, tag = "1")]
    pub player: PlayerInfo,
    #[prost(string, required, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60028)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs60028 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
    #[prost(string, required, tag = "2")]
    pub keyword: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60003)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60003 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60011)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60011 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60012)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60012 {
    #[prost(uint32, required, tag = "1")]
    pub player_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub duty_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60018)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60018 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60023)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60023 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60025)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60025 {
    #[prost(message, repeated, tag = "1")]
    pub guild_list: ::prost::alloc::vec::Vec<GuildSimpleInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60030)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60030 {
    #[prost(message, required, tag = "1")]
    pub guild: GuildBaseInfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60031)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60031 {
    #[prost(message, repeated, tag = "1")]
    pub member_list: ::prost::alloc::vec::Vec<MemberInfo>,
    #[prost(message, repeated, tag = "2")]
    pub log_list: ::prost::alloc::vec::Vec<LogInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60032)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60032 {
    #[prost(uint32, required, tag = "1")]
    pub lv: u32,
    #[prost(uint32, required, tag = "2")]
    pub exp: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60036)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60036 {
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60017)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60017 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60007)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs60007 {
    #[prost(string, required, tag = "1")]
    pub chat: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60016)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60016 {
    #[prost(uint32, required, tag = "1")]
    pub player_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60000)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60000 {
    #[prost(message, required, tag = "1")]
    pub guild: GuildInfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60100)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60100 {
    #[prost(uint32, required, tag = "1")]
    pub count: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GoodsInfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
    #[prost(uint32, required, tag = "3")]
    pub index: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60001)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs60001 {
    #[prost(uint32, required, tag = "2")]
    pub policy: u32,
    #[prost(uint32, required, tag = "1")]
    pub faction: u32,
    #[prost(string, required, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub manifesto: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60019)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60019 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60026)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs60026 {
    #[prost(uint32, optional, tag = "2")]
    pub int: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
    #[prost(string, optional, tag = "3")]
    pub str: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60034)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60034 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "2")]
    pub info: ShopInfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60015)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60015 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60035)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs60035 {
    #[prost(message, repeated, tag = "3")]
    pub selected: ::prost::alloc::vec::Vec<GuildShopInfo>,
    #[prost(uint32, required, tag = "2")]
    pub index: u32,
    #[prost(uint32, required, tag = "1")]
    pub goodsid: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60008)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60008 {
    #[prost(message, required, tag = "1")]
    pub chat: GuideChat,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60021)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60021 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogInfo {
    #[prost(uint32, required, tag = "2")]
    pub time: u32,
    #[prost(uint32, optional, tag = "5")]
    pub arg1: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "3")]
    pub user_id: u32,
    #[prost(uint32, required, tag = "1")]
    pub cmd: u32,
    #[prost(string, required, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildExpansionInfo {
    #[prost(message, repeated, tag = "4")]
    pub technologys: ::prost::alloc::vec::Vec<super::guild::GuildTechnology>,
    #[prost(message, required, tag = "2")]
    pub this_weekly_tasks: super::guild::WeeklyTask,
    #[prost(uint32, required, tag = "5")]
    pub retreat_cnt: u32,
    #[prost(uint32, required, tag = "7")]
    pub last_benefit_finish_time: u32,
    #[prost(uint32, required, tag = "8")]
    pub active_event_cnt: u32,
    #[prost(uint32, required, tag = "3")]
    pub benefit_finish_time: u32,
    #[prost(uint32, required, tag = "1")]
    pub capital: u32,
    #[prost(uint32, required, tag = "6")]
    pub tech_cancel_cnt: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildSimpleInfo {
    #[prost(uint32, required, tag = "3")]
    pub tech_seat: u32,
    #[prost(message, required, tag = "1")]
    pub base: GuildBaseInfo,
    #[prost(message, required, tag = "2")]
    pub leader: PlayerInfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60013)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60013 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60009)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60009 {
    #[prost(uint32, required, tag = "1")]
    pub count: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberInfo {
    #[prost(string, required, tag = "6")]
    pub adv: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "7")]
    pub online: u32,
    #[prost(uint32, required, tag = "3")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub duty: u32,
    #[prost(uint32, required, tag = "8")]
    pub pre_online_time: u32,
    #[prost(message, optional, tag = "9")]
    pub display: ::core::option::Option<super::common::Displayinfo>,
    #[prost(uint32, required, tag = "12")]
    pub join_time: u32,
    #[prost(uint32, required, tag = "5")]
    pub lv: u32,
    #[prost(uint32, required, tag = "1")]
    pub liveness: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildInfo {
    #[prost(message, repeated, tag = "2")]
    pub member: ::prost::alloc::vec::Vec<MemberInfo>,
    #[prost(message, required, tag = "1")]
    pub base: GuildBaseInfo,
    #[prost(message, repeated, tag = "3")]
    pub log: ::prost::alloc::vec::Vec<LogInfo>,
    #[prost(message, required, tag = "4")]
    pub guild_ex: GuildExpansionInfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60006)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60006 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60027)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc60027 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopInfo {
    #[prost(message, repeated, tag = "3")]
    pub good_list: ::prost::alloc::vec::Vec<GoodsInfo>,
    #[prost(uint32, required, tag = "1")]
    pub refresh_count: u32,
    #[prost(uint32, required, tag = "2")]
    pub next_refresh_time: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60102)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60102 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60005)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs60005 {
    #[prost(string, required, tag = "2")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60022)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60022 {
    #[prost(uint32, required, tag = "1")]
    pub player_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInfo {
    #[prost(message, required, tag = "2")]
    pub player: PlayerInfo,
    #[prost(string, required, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "1")]
    pub timestamp: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildBaseInfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "8")]
    pub exp: u32,
    #[prost(uint32, required, tag = "2")]
    pub policy: u32,
    #[prost(uint32, required, tag = "9")]
    pub member_count: u32,
    #[prost(string, required, tag = "6")]
    pub announce: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "5")]
    pub level: u32,
    #[prost(uint32, required, tag = "11")]
    pub kick_leader_cd: u32,
    #[prost(uint32, required, tag = "10")]
    pub change_faction_cd: u32,
    #[prost(string, required, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "3")]
    pub faction: u32,
    #[prost(string, required, tag = "7")]
    pub manifesto: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60029)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60029 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub guild: ::prost::alloc::vec::Vec<GuildSimpleInfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GuildShopInfo {
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60103)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc60103 {
    #[prost(message, required, tag = "1")]
    pub user_info: UserGuildInfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(60024)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs60024 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(message, optional, tag = "4")]
    pub display: ::core::option::Option<super::common::Displayinfo>,
    #[prost(uint32, required, tag = "3")]
    pub lv: u32,
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
}