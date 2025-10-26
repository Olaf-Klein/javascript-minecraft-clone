// Example mod demonstrating the host API
// This mod runs every tick, prints the block under the player, and
// attempts to spawn a test item into the player's inventory once every 5 seconds.

let tick_count = 0;

fn on_tick() {
    tick_count = tick_count + 1;

    // Get player position from host and round to block coordinates
    let pos = host_get_player_pos();
    let x = pos.0 as int;
    let y = pos.1 as int - 1; // block beneath the player
    let z = pos.2 as int;

    // Read block id at that position and log it
    let id = host_get_block(x, y, z);
    log("Player standing above block id: " + id.to_string());

    // Every 300 ticks (~5s at 60Hz), try to spawn a test item into inventory
    if tick_count % 300 == 0 {
        let ok = host_spawn_item("test_item", 1);
        if ok {
            log("Spawned test_item into inventory");
        } else {
            log("Failed to spawn test_item (inventory full)");
        }
        // Also set the time of day to midday as a demo
        host_set_time(6000.0);
    }
}
