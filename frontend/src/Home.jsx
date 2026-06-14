import "./Home.css";
import { Link } from "react-router-dom";

export default function Home() {
    return (
        <>
            <div>home</div>
            <Link to="/signup">Signup</Link>
            <Link to="/login">Login</Link>
        </>
    );
}