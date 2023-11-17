use rapier3d::prelude::*;

pub struct Scene{
    pub collider_set: Option<ColliderSet>,
    pub rigidbody_set: Option<RigidBodySet>,
    pub impulse_joint_set: Option<ImpulseJointSet>,
    pub multibody_joint_set: Option<MultibodyJointSet>,

    integration_params: Option<IntegrationParameters>,
    physics_pipeline: Option<PhysicsPipeline>,
    island_manager: Option<IslandManager>,
    broad_phase: Option<BroadPhase>,
    narrow_phase: Option<NarrowPhase>,
    ccd_solver: Option<CCDSolver>,

    physics_hooks: (),
    event_handlers: (),

    gravity: rapier3d::na::Vector3<f32>,

    existing_colliders: Vec<ColliderHandle>

}

impl Scene{
    pub fn new() -> Scene{
        Scene{
            collider_set: None,
            rigidbody_set: None,
            impulse_joint_set: None,
            multibody_joint_set: None,
            integration_params: None,
            physics_pipeline: None,
            island_manager: None,
            broad_phase: None,
            narrow_phase: None,
            ccd_solver: None,
            physics_hooks: (),
            event_handlers: (),
            gravity: vector![0.0, -9.8, 0.0],
            existing_colliders: std::vec![]
        }
    }

    pub fn create(mut self) -> Self{
        self.collider_set = Some(ColliderSet::new());
        self.rigidbody_set = Some(RigidBodySet::new());
        self.impulse_joint_set = Some(ImpulseJointSet::new());
        self.multibody_joint_set = Some(MultibodyJointSet::new());

        self.integration_params = Some(IntegrationParameters::default());
        self.physics_pipeline = Some(PhysicsPipeline::new());
        self.island_manager = Some(IslandManager::new());
        self.broad_phase = Some(BroadPhase::new());
        self.narrow_phase = Some(NarrowPhase::new());

        self.ccd_solver = Some(CCDSolver::new());

        self
    }

    pub fn step(&mut self, dt: f32){
        self.integration_params
            .as_mut()
            .expect("Couldn't get integration params")
            .dt = dt;

        let mut dead_handles: Vec<ColliderHandle> = std::vec![];
        for handle in self.collider_set.as_mut().unwrap().iter(){
            if !self.existing_colliders.contains(&handle.0){
                dead_handles.push(handle.0);
            }
        }

        for handle in dead_handles{
            self.collider_set.as_mut().unwrap()
                .remove(handle, self.island_manager.as_mut().unwrap(),
                self.rigidbody_set.as_mut().unwrap(),
                false);
        }

        self.existing_colliders.clear();

        self.physics_pipeline
            .as_mut()
            .expect("Couldn't get physics pipeline!")
            .step(
                &self.gravity,
                self.integration_params.as_ref().unwrap(),
                self.island_manager.as_mut().unwrap(),
                self.broad_phase.as_mut().unwrap(),
                self.narrow_phase.as_mut().unwrap(),
                self.rigidbody_set.as_mut().unwrap(),
                self.collider_set.as_mut().unwrap(),
                self.impulse_joint_set.as_mut().unwrap(),
                self.multibody_joint_set.as_mut().unwrap(),
                self.ccd_solver.as_mut().unwrap(),
                None,
                &self.physics_hooks,
                &self.event_handlers
            );
    }

    pub fn signal_existence(&mut self, handle: ColliderHandle){
        self.existing_colliders.push(handle);
    }


}