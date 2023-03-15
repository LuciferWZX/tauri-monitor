import {invoke} from "@tauri-apps/api";
import {ProcessData} from "@/ts/monitor";
import processInfoState from "@/store/processInfo"
import {useLayoutEffect} from "react";
const useProcessInfo = (config?:{
    interval?:number
}) => {
    const handleProcessInfo=async ()=>{
        const processInfo = await invoke<ProcessData[]>("process_info",{})
        processInfoState.state.processDataList = processInfo.slice(0,10).map(_data=>{
            return {
                ..._data,
                memory:(_data.memory *1024)
            }
        })
    }
    useLayoutEffect(()=>{
        const interval = setInterval(()=>{
            handleProcessInfo()
        },config?.interval ?? 3000)
        return ()=>{
            clearInterval(interval)
        }
    },[])
}
export default useProcessInfo