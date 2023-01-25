// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from "next"
import { getLatestWakatimeAsync, getWakatimeExePathAsync } from "../../../utils/wakatime"
import { spawnAsync } from "../../../utils/process"

const handlerAsync = async (_req: NextApiRequest, res: NextApiResponse) => {
    const updateRes = await getLatestWakatimeAsync()
    if (updateRes.isErr) {
        res.status(500).send(updateRes.unwrapErr())
        return
    }

    const cliPathRes = await getWakatimeExePathAsync()
    if (cliPathRes.isErr) {
        res.status(500).send(cliPathRes.unwrapErr())
        return
    }

    const cliPath = cliPathRes.unwrap()

    const std = await spawnAsync(cliPath, ["--write", "--plugin", "vscode/1.0.0", "--entity-type", "file", "--time", "1", "--project", "vscode", "--entity", "test.ts", "--key", "test"])

    res.send(JSON.stringify(std))
}

export default handlerAsync
