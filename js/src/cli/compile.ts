import { AxiomCircuit } from "../js";
import { getFunctionFromTs, getProvider, readJsonFromFile, saveJsonToFile } from "./utils";

export const compile = async (path: string, options: { stats: boolean, output: string, provider?: string, inputs?: string }) => {
    const f = await getFunctionFromTs(path);
    const provider = getProvider(options.provider);
    const circuit = new AxiomCircuit({
        f: f.circuit,
        mock: true,
        provider,
    })
    let circuitInputs = f.inputs;
    if (options.inputs) {
        circuitInputs = readJsonFromFile(options.inputs);
    }
    try {
        const res = await circuit.compile(circuitInputs);
        saveJsonToFile(JSON.stringify(res), options.output, "build.json");
    }
    catch (e) {
        console.error(e);
    }
}