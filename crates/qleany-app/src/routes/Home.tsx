import {loadManifest} from "../controller/handling_manifest_controller.ts";

const Home = () => {

    async function handleOpenManifest() {

        const dto = {
            manifest_path: "C:\\Users\\cyril\\Devel\\qleany\\qleany.yaml"
        }
        loadManifest(dto).catch(e => console.error(e));
    }

    return (
        <div className="flex-1 p-4">
            <h1 className="text-4xl font-bold underline">Home</h1>
            <p className="text-lg">Welcome to Qleany!</p>
            <button onClick={handleOpenManifest}>Open manifest</button>
            <button className="btn">Close current manifest</button>

        </div>
    );
}

export default Home;