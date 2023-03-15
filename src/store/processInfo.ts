import {ProcessData} from "@/ts/monitor";
import {proxy} from "umi";

type ProcessInfoState = {
    processDataList:ProcessData[]
}
const initialState:ProcessInfoState = {
  processDataList:[]
}
const state = proxy(initialState)
export default {
    state
}