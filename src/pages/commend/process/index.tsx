import {FC} from "react";
import useProcessInfo from "@/pages/commend/process/useProcessInfo";
import {useSnapshot} from "umi";
import {Button, Progress, Space} from "antd";
import processInfoState from "@/store/processInfo"
import styled from "styled-components";
const Process:FC = () => {
    const {processDataList}=useSnapshot(processInfoState.state)
    useProcessInfo()
    console.log(222,processDataList)
    return(
        <StyledProcess>
            <Space direction="vertical">
                {processDataList.slice(0,10).map(data=>{
                    return(
                        <div>
                            <Progress style={{width:200}} key={data.pid} percent={18}  />
                            {data.name}
                        </div>
                    )
                })}
            </Space>
        </StyledProcess>
    )
}
export default Process
const StyledProcess = styled.div`
    width: 100%;
`