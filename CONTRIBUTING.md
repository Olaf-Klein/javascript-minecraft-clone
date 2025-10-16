# Contributing to JavaScript Minecraft Clone

Thank you for considering contributing to this project!

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in Issues
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Node version, etc.)

### Suggesting Features

1. Check if the feature has already been suggested
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Possible implementation approach

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**:
   - Follow the existing code style
   - Add comments for complex logic
   - Update documentation if needed

4. **Test your changes**:
   - Ensure the server starts correctly
   - Test client functionality
   - Verify no regressions

5. **Commit your changes**:
   ```bash
   git commit -m "Add feature: your feature description"
   ```

6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**:
   - Provide a clear description
   - Reference any related issues
   - Add screenshots for UI changes

## Code Style

- Use 2 spaces for indentation
- Use single quotes for strings
- Add semicolons at the end of statements
- Follow ESLint rules (run `npm run lint`)

## Project Structure

```
├── client/          # Client-side code
│   ├── src/         # Source files
│   └── index.html   # Entry HTML
├── server/          # Server-side code
│   ├── world/       # World generation/management
│   ├── network/     # Networking
│   └── plugins/     # Plugin system
├── shared/          # Shared code
│   ├── blocks/      # Block registry
│   ├── protocol/    # Network protocol
│   └── constants/   # Constants
└── docs/            # Documentation
```

## Development Workflow

1. **Setup**:
   ```bash
   npm install
   ```

2. **Start server**:
   ```bash
   npm start
   ```

3. **Start client** (in another terminal):
   ```bash
   npm run client:dev
   ```

4. **Lint**:
   ```bash
   npm run lint
   ```

## Adding New Features

### New Block Types

1. Add block to `shared/blocks/registry.js`
2. Add texture/material in `client/src/engine/renderer.js`
3. Update world generator if needed

### New Packets

1. Define packet type in `shared/protocol/packets.js`
2. Add handler in server (`server/network/server.js`)
3. Add handler in client (`client/src/network/client.js`)

### New Plugins

1. Create plugin directory in `server/plugins/plugins-data/`
2. Add `plugin.json` manifest
3. Implement `main.js`
4. Test with example server

## Testing

Currently, the project uses manual testing. We welcome contributions for:
- Unit tests
- Integration tests
- End-to-end tests

## Documentation

When adding features, please update:
- README.md (if it affects setup/usage)
- Architecture documentation (for major changes)
- API documentation (for plugin API changes)
- Inline code comments

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to ask questions by:
- Opening a discussion
- Commenting on issues
- Reaching out to maintainers

Thank you for contributing! 🎮
