import ImageList from "@mui/material/ImageList/ImageList";
import ImageListItem from "@mui/material/ImageListItem/ImageListItem";
import DeleteSharpIcon from '@mui/icons-material/DeleteSharp';
import HomeIcon from '@mui/icons-material/HomeSharp';
import React from "react";
import ReactDOM from "react-dom/client";

const App = () => {
    return (
        <>
            <ImageList sx={{ height: 450 }} cols={4} rowHeight={164}>
                <ImageListItem key="A">
                    <DeleteSharpIcon sx={{ fontSize: 200 }} />
                </ImageListItem>
                <ImageListItem key="B">
                    <HomeIcon sx={{ fontSize: 200 }} />
                </ImageListItem>
                <ImageListItem key="C">
                    <DeleteSharpIcon sx={{ fontSize: 200 }} />
                </ImageListItem>
                <ImageListItem key="D">
                    <DeleteSharpIcon sx={{ fontSize: 200 }} />
                </ImageListItem>
            </ImageList>
        </>)
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
