/**
 * Example plugin demonstrating the plugin API
 */

exports.onEnable = function() {
  console.log('Example plugin enabled!');
  
  // Load configuration
  const config = api.getConfig();
  const welcomeMessage = config.welcomeMessage || 'Welcome to the server!';
  
  // Register event handlers
  api.on('playerJoin', (data) => {
    api.broadcast(`§e${data.username} joined the game!`);
    api.sendMessage(data.uuid, welcomeMessage);
  });
  
  api.on('playerLeave', (data) => {
    api.broadcast(`§e${data.username} left the game!`);
  });
  
  api.on('chatMessage', (data) => {
    console.log(`Chat: <${data.username}> ${data.message}`);
  });
  
  api.on('blockPlace', (data) => {
    const player = api.getPlayer(data.uuid);
    if (player) {
      console.log(`${player.username} placed block ${data.blockId} at ${data.x},${data.y},${data.z}`);
    }
  });
  
  // Register commands
  api.registerCommand('info', (player, args) => {
    const players = api.getPlayers();
    api.sendMessage(player.uuid, `§aServer Info:`);
    api.sendMessage(player.uuid, `§7Players online: ${players.length}`);
    api.sendMessage(player.uuid, `§7Your position: ${Math.floor(player.position.x)}, ${Math.floor(player.position.y)}, ${Math.floor(player.position.z)}`);
  });
  
  api.registerCommand('broadcast', (player, args) => {
    if (args.length === 0) {
      api.sendMessage(player.uuid, '§cUsage: /broadcast <message>');
      return;
    }
    const message = args.join(' ');
    api.broadcast(`§6[Broadcast] §f${message}`);
  });
  
  console.log('Example plugin commands registered: /info, /broadcast');
};

exports.onDisable = function() {
  console.log('Example plugin disabled!');
};
