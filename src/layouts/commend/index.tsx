import React, {FC} from "react";
import {Outlet} from "umi";

const CommendLayout:FC = () => {
    return(
        <div>
            <Outlet/>
        </div>
    )
}
export default CommendLayout