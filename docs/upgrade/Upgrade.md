### Recompile and connect to the local node

```sh
cargo build --release --package node-template-runtime
```

With this command, the build artifacts are output to the *target/release* directory. The WebAssembly build artifacts are in the *target/release/wbuild/node-template-runtime* directory. For example, you should see the following WebAssembly artifacts:

```sh
node_template_runtime.compact.compressed.wasm
node_template_runtime.compact.wasm
node_template_runtime.wasm
```

### Execute a runtime upgrade

To update the network with the upgraded runtime:

1. Open the **Polkadot/Substrate Portal** in a browser and connect to the local node.
2. Click **Developer** and select **Extrinsics** to submit a transaction for the runtime to use the new build artifact.
3. Select the administrative **Alice** account.
4. Select the **sudo** pallet and the **sudoUncheckedWeight(call, weight)** function.
5. Select **system** and **setCode(code)** as the call to make using the Alice account.
6. Click **file upload**, then select or drag and drop the compact and compressed WebAssembly file—node_template_runtime.compact.compressed.wasm—that you generated for the updated runtime.

    For example, navigate to the `target/release/wbuild/node-template-runtime` directory and select `node_template_runtime.compact.compressed.wasm` as the file to upload.

7. Leave both of the **weight** parameters set to the default value of 0.

![Polkadot/Substrate Portal](./set-code-transaction.avif)

8. Click **Submit Transaction**.
9. Review the authorization, then click **Sign and Submit**.
10. Click **Network** and select **Explorer** to see that there has been a successful `sudo.Sudid` event.

![Polkadot/Substrate Portal](./set-code-sudo-event.avif)