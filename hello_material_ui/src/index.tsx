import React from "react";
import ReactDOM from "react-dom/client";
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import FormControlLabel from "@mui/material/FormControlLabel/FormControlLabel";
import Checkbox from "@mui/material/Checkbox/Checkbox";
import Toolbar from "@mui/material/Toolbar/Toolbar";
import IconButton from "@mui/material/IconButton/IconButton";
import Typography from "@mui/material/Typography/Typography";

const App = () => {
    const [checked, setChecked] = React.useState(true);

    return (
        <Box>
            <AppBar position="static" >
                <Toolbar>
                    <IconButton
                        size="large"
                        edge="start"
                        color="inherit"
                        aria-label="menu"
                        sx={{ mr: 2 }}
                    >
                        {/* ボタン */}
                        <Button variant="contained">Hello world</Button>
                    </IconButton>
                    <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                        Hello World
                    </Typography>
                    <Button color="inherit">Login</Button>
                </Toolbar>
            </AppBar>


            {/* ステートを利用したチェックボックス */}
            <FormControlLabel
                label="Hello Checkbox"
                control={
                    <Checkbox
                        checked={checked}
                        onChange={event => setChecked(event.target.checked)}
                    />
                }
            />
        </Box>);
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
