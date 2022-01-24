use glam::{Quat, Vec3};
use std::{ffi::c_void, os::raw::c_char};

#[repr(u8)]
#[derive(Debug)]
pub enum ResultCode {
    Success = 0,
    Panic = 1,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entity {
    pub id: u64,
}
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Quat> for Quaternion {
    fn from(v: Quat) -> Self {
        Quaternion {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}
impl Into<Quat> for Quaternion {
    fn into(self) -> Quat {
        Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }
}

impl Into<Vec3> for Vector3 {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl From<Vec3> for Vector3 {
    fn from(v: Vec3) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

// TODO: Is there a more typesafe way of defining an opaque type that
// is c ffi safe in Rust without nightly?
pub type AActorOpaque = c_void;
pub type UPrimtiveOpaque = c_void;
pub type UCapsuleOpaque = c_void;

pub type GetSpatialDataFn = extern "C" fn(
    actor: *const AActorOpaque,
    position: &mut Vector3,
    rotation: &mut Quaternion,
    scale: &mut Vector3,
);

pub type LogFn = extern "C" fn(*const c_char, i32);

pub type SetSpatialDataFn = extern "C" fn(
    actor: *mut AActorOpaque,
    position: Vector3,
    rotation: Quaternion,
    scale: Vector3,
);
pub type IterateActorsFn = extern "C" fn(array: *mut *mut AActorOpaque, len: *mut u64);
pub type GetActionStateFn = extern "C" fn(name: *const c_char, state: &mut ActionState);
pub type GetAxisValueFn = extern "C" fn(name: *const c_char, len: usize, value: &mut f32);
pub type SetEntityForActorFn = extern "C" fn(name: *mut AActorOpaque, entity: Entity);
pub type SpawnActorFn = extern "C" fn(
    actor_class: ActorClass,
    position: Vector3,
    rotation: Quaternion,
    scale: Vector3,
) -> *mut AActorOpaque;
pub type SetViewTargetFn = extern "C" fn(actor: *const AActorOpaque);
pub type GetMouseDeltaFn = extern "C" fn(x: &mut f32, y: &mut f32);
pub type GetActorComponentsFn =
    extern "C" fn(actor: *const AActorOpaque, data: *mut ActorComponentPtr, len: &mut usize);
pub type VisualLogSegmentFn =
    extern "C" fn(owner: *const AActorOpaque, start: Vector3, end: Vector3, color: Color);

extern "C" {
    pub fn SetSpatialData(
        actor: *mut AActorOpaque,
        position: Vector3,
        rotation: Quaternion,
        scale: Vector3,
    );

    pub fn GetSpatialData(
        actor: *const AActorOpaque,
        position: &mut Vector3,
        rotation: &mut Quaternion,
        scale: &mut Vector3,
    );
    pub fn TickActor(actor: *mut AActorOpaque, dt: f32);
    pub fn Log(s: *const c_char, len: i32);
    pub fn IterateActors(array: *mut *mut AActorOpaque, len: *mut u64);
    pub fn GetActionState(name: *const c_char, state: &mut ActionState);
    pub fn GetAxisValue(name: *const c_char, len: usize, value: &mut f32);
    pub fn SetEntityForActor(name: *mut AActorOpaque, entity: Entity);
    pub fn SpawnActor(
        actor_class: ActorClass,
        position: Vector3,
        rotation: Quaternion,
        scale: Vector3,
    ) -> *mut AActorOpaque;
    pub fn SetViewTarget(actor: *const AActorOpaque);
    pub fn GetMouseDelta(x: &mut f32, y: &mut f32);
    pub fn GetActorComponents(
        actor: *const AActorOpaque,
        data: *mut ActorComponentPtr,
        len: &mut usize,
    );
    pub fn VisualLogSegment(owner: *const AActorOpaque, start: Vector3, end: Vector3, color: Color);
}

#[repr(C)]
pub struct UnrealBindings {
    pub get_spatial_data: GetSpatialDataFn,
    pub set_spatial_data: SetSpatialDataFn,
    pub log: LogFn,
    pub iterate_actors: IterateActorsFn,
    pub get_action_state: GetActionStateFn,
    pub get_axis_value: GetAxisValueFn,
    pub set_entity_for_actor: SetEntityForActorFn,
    pub spawn_actor: SpawnActorFn,
    pub set_view_target: SetViewTargetFn,
    pub get_mouse_delta: GetMouseDeltaFn,
    pub get_actor_components: GetActorComponentsFn,
    pub visual_log_segment: VisualLogSegmentFn,
    pub physics_bindings: UnrealPhysicsBindings,
}
unsafe impl Sync for UnrealBindings {}
unsafe impl Send for UnrealBindings {}

#[repr(u8)]
#[derive(Debug)]
pub enum ActionState {
    Pressed = 0,
    Released = 1,
    Held = 2,
    Nothing = 3,
}
#[repr(u32)]
#[derive(Debug)]
pub enum ActorClass {
    RustActor = 0,
    CameraActor = 1,
}
#[repr(u32)]
#[derive(Debug)]
pub enum ActorComponentType {
    Primitive,
}

#[repr(C)]
#[derive(Debug)]
pub struct ActorComponentPtr {
    pub ty: ActorComponentType,
    pub ptr: *mut c_void,
}

#[repr(C)]
pub struct Uuid {
    pub bytes: [u8; 16],
}

pub type EntryUnrealBindingsFn = extern "C" fn(bindings: UnrealBindings) -> RustBindings;
pub type BeginPlayFn = extern "C" fn() -> ResultCode;
pub type TickFn = extern "C" fn(dt: f32) -> ResultCode;
pub type RetrieveUuids = unsafe extern "C" fn(ptr: *mut Uuid, len: *mut usize);
pub type GetVelocityRustFn =
    unsafe extern "C" fn(actor: *const AActorOpaque, velocity: &mut Vector3);

pub type CollisionShape = c_void;
#[repr(C)]
pub struct RustBindings {
    pub retrieve_uuids: RetrieveUuids,
    pub get_velocity: GetVelocityRustFn,
    pub tick: TickFn,
    pub begin_play: BeginPlayFn,
}
pub type GetVelocityFn = extern "C" fn(primitive: *const UPrimtiveOpaque) -> Vector3;
pub type SetVelocityFn = extern "C" fn(primitive: *mut UPrimtiveOpaque, velocity: Vector3);
pub type IsSimulatingFn = extern "C" fn(primitive: *const UPrimtiveOpaque) -> u32;
pub type AddForceFn = extern "C" fn(actor: *mut UPrimtiveOpaque, force: Vector3);
pub type AddImpulseFn = extern "C" fn(actor: *mut UPrimtiveOpaque, force: Vector3);
pub type LineTraceFn = extern "C" fn(start: Vector3, end: Vector3, result: &mut HitResult) -> u32;
#[repr(C)]
pub struct UnrealPhysicsBindings {
    pub get_velocity: GetVelocityFn,
    pub set_velocity: SetVelocityFn,
    pub is_simulating: IsSimulatingFn,
    pub add_force: AddForceFn,
    pub add_impulse: AddImpulseFn,
    pub line_trace: LineTraceFn,
}
#[repr(C)]
pub struct HitResult {
    pub actor: *mut AActorOpaque,
    pub distance: f32,
    pub normal: Vector3,
    pub location: Vector3,
    pub impact_location: Vector3,
    pub pentration_depth: f32,
}

extern "C" {
    pub fn GetVelocity(primitive: *const UPrimtiveOpaque) -> Vector3;
    pub fn SetVelocity(primitive: *mut UPrimtiveOpaque, velocity: Vector3);
    pub fn IsSimulating(primitive: *const UPrimtiveOpaque) -> u32;
    pub fn AddForce(actor: *mut UPrimtiveOpaque, force: Vector3);
    pub fn AddImpulse(actor: *mut UPrimtiveOpaque, force: Vector3);
    pub fn LineTrace(start: Vector3, end: Vector3, result: &mut HitResult) -> u32;
}