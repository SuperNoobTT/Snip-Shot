use bevy::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, KinematicCharacterController, Sensor, Sleeping};
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
    pub fn random_position(&mut self, min: Vec2, max: Vec2) -> Vec2 {
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
    //TODO: maybe move this to the movement component?
    mut update_timer: Local<f32>,
    mut randomiser: ResMut<PositionRandomiser>,
    time: Res<Time>,
) {
    //Get the player's translation to move all enemies towards it
    let Ok(&Transform{translation: plyr_trans, ..}) = player_query.get_single() else {
        return;
    };

    const UPDATE_TIME: f32 = 5.0;

    let mut dir_update: bool = false;
    if *update_timer < 0.0 {
        *update_timer = UPDATE_TIME;
        dir_update = true;
    } else {
        *update_timer -= time.delta_seconds();
    }

    for (
        mut movement, 
        &Transform{translation: trans, ..}, 
        mut controller
    ) in enemy_query.iter_mut() 
    {
        if dir_update {
            let travel_vec = trans - plyr_trans;
            movement.direction = travel_vec;
        }
        //Add some random offset for spoopy !
        let offset = randomiser.random_position(Vec2::splat(-1.0), Vec2::splat(1.0));
        let mut base_trans = movement.direction;
        base_trans.x += offset.x;
        base_trans.y += offset.y;

        let translation = (base_trans.normalize_or_zero()) * movement.base_speed * time.delta_seconds();
        controller.translation = Some(translation)
    }
}

pub(crate) fn check_collision(
    mut intersection_evs: EventReader<CollisionEvent>,
    enemy_query: Query<&Parent, With<Sensor>>,
    mut assimilation_query: Query<(&mut Sleeping, &mut Visibility), With<Enemy>>,
    mut commands: Commands
) {
    let mut enemies: Vec<&Entity> = Vec::new();
    let mut platforms: Vec<&Entity> = Vec::new();
    
    for event in intersection_evs.read() {
        if let CollisionEvent::Started(entity1, entity2, CollisionEventFlags::SENSOR) = event {
            if let Ok(enemy) = enemy_query.get(*entity1) {
                //FIXME: We're pushing enemy: &Parent, but is that the same as &Entity?
                enemies.push(enemy);
                platforms.push(entity2);
            } else if let Ok(enemy) = enemy_query.get(*entity2) {
                enemies.push(enemy);
                platforms.push(entity1);
            }
        }
    }
    assimilate_enemies_with_platforms(&mut commands, &mut assimilation_query, enemies, platforms);
}

fn assimilate_enemies_with_platforms(
    commands: &mut Commands,
    sleepers: &mut Query<(&mut Sleeping, &mut Visibility), With<Enemy>>,
    enemies: Vec<&Entity>,
    platforms: Vec<&Entity>
) {
    for (&enemy, &platform) in enemies.into_iter().zip(platforms.into_iter()) {
        //Parent enemy to platform
        commands.entity(platform).push_children(&[enemy]);
        let (mut sleep, mut vis) = sleepers.get_mut(enemy).expect("There should be an enemy available to assimilate!");
        //Disable physics & hide enemy
        sleep.sleeping = true;
        *vis = Visibility::Hidden;
        //TODO: Consider moving the enemy to `inside` the platform 
        //e.g. using Query<&mut Transform, Or<(With<Enemy>, With<PlatformMarker>)>> (This requires a marker for platform)
    }
}

