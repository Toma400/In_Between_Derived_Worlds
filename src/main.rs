use bevy::prelude::*;

const ibdw_ver: f32 = 0.1; // IBDW version

fn main() {

    fn greet() { println!("Welcome in In Between Derived Worlds v:{}", ibdw_ver); }

    #[derive(Component)]
    struct LivingEntity;

    #[derive(Component)]
    struct Name(String);

    fn add_entities(mut commands: Commands) {
        commands.spawn((LivingEntity, Name("John Harperwood".to_string())));
        commands.spawn((LivingEntity, Name("Ayaan Siyad".to_string())));
        commands.spawn((LivingEntity, Name("Vei Nai".to_string())));
    }

    fn greet_entities(query: Query<&Name, With<LivingEntity>>) {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }

    App::new()
        .add_startup_system(add_entities)
        .add_system(greet)
        .add_system(greet_entities)
        .run();
}
