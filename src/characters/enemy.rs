use bevy::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, KinematicCharacterController, Sensor};
use super::player::{CharacterBundle, Player};
use super::components::Movement;
use crate::utils::{ENEMY_COLLISION, ENEMY_SOLVER};

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct EnemySpawnTimer {
    pub(crate) timer: Timer
}

pub(crate) fn setup_timer(
    mut commands: Commands
) {
    //TODO: Add sensor collider to collide with platforms!
    commands.spawn(EnemySpawnTimer{timer: Timer::from_seconds(5.0, TimerMode::Repeating)});
}

#[derive(Resource, Debug, Clone)]
pub struct PositionRandomiser {
    seed: u64
}

impl Default for PositionRandomiser {
    fn default() -> Self {
        Self { seed: random() }
    }
}

impl PositionRandomiser {
    pub fn random_position(&self, min: Vec2, max: Vec2) -> Vec2 {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        Vec2::new(
            rng.gen_range(min.x..max.x),
            rng.gen_range(min.y..max.y)
        )
    }

    pub fn update_seed(&mut self) {
        self.seed = rand::random();
    }
}

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct Enemy;

pub(crate) fn gen_enemies(
    mut commands: Commands,
    mut timer_query: Query<&mut EnemySpawnTimer>,
    mut randomiser: ResMut<PositionRandomiser>,
    time: Res<Time>
) {
    let enemy_body: Collider = Collider::capsule_y(3.5, 1.0);
    //Assuming we might want to have multiple spawners
    for mut enemy_timer in timer_query.iter_mut() {
        let timer = &mut enemy_timer.timer;
        timer.tick(time.delta());
        if timer.finished() {
            let position = randomiser.random_position(Vec2::new(0.0, 0.0), Vec2::new(800.0, 600.0));
            commands.spawn((
                CharacterBundle::<Enemy>::from_colliders(
                    Some(ENEMY_COLLISION),
                    enemy_body.clone(),
                ),
                ENEMY_SOLVER
            )).with_children(|parent| {
                parent.spawn((
                    enemy_body.clone(),
                    ENEMY_SOLVER,
                    Sensor
                ));
            });
        }
    }
}

pub(crate) fn move_enemies(
    mut enemy_query: Query<(&mut Movement, &Transform, &mut KinematicCharacterController), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let plyr_trans = player_query.single() else {
        return;
    };

    for (mut movement, transform, mut controller) in enemy_query.iter_mut() {
        todo!()
    }
}

pub(crate) fn check_collision(
    mut intersection_evs: EventReader<CollisionEvent>,
    enemy_query: Query<&Parent, With<Sensor>>,
    mut commands: Commands
) {
    let mut enemies: Vec<&Entity> = Vec::new();
    let mut platforms: Vec<&Entity> = Vec::new();
    
    for event in intersection_evs.read() {
        if let CollisionEvent::Started(entity1, entity2, CollisionEventFlags::SENSOR) = event {
            if let Ok(enemy) = enemy_query.get(*entity1) {
                enemies.push(enemy);
                platforms.push(entity2);
            } else if let Ok(enemy) = enemy_query.get(*entity2) {
                enemies.push(enemy);
                platforms.push(entity1);
            }
        }
    }
    assimilate_enemies_with_platforms(commands, enemies, platforms);

}

fn assimilate_enemies_with_platforms(
    mut commands: Commands,
    enemies: Vec<&Entity>,
    platforms: Vec<&Entity>
) {
    for (enemy, platform) in enemies.into_iter().zip(platforms.into_iter()) {

    }
}