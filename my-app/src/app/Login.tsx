import { FormEvent, useState } from 'react';
import styles from './Login.module.css'
import axios from 'axios';
import router from 'next/router';

interface LoginForm {
    email: string;
    password: string;
}

const Login: React.FC = () => {
    const [email, setEmail] = useState<string>('');
    const [password, setPassword] = useState<string>('');

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {id, value} = e.target;
        if (id === 'email') {
            setEmail(value);
        } else if (id === 'password') {
            setPassword(value);
        }
    };

const handleSubmit = async(e: FormEvent) => {
        e.preventDefault();
        const form: LoginForm = {email, password}

        try {
            const response = await axios.post('http://localhost:8080/login', form, {
                headers: {
                    'Content-Type': 'application/json',
                },
                withCredentials: true,
            });
            if (response.status === 200) {
                await router.push('http://localhost:3001/profile');
            } else {
                console.log('Request failed to login');
            }
        } catch (error) {
            console.log('failed to login:', error);
        }
    }

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>
                <div className={styles.field}>
                    <label htmlFor="email" className={styles.label}>
                        Email:
                    </label>
                    <input
                    type="email"
                    id="email"
                    value={email}
                    onChange={handleInputChange}
                    className={styles.input}
                    />
                </div>
                <div className={styles.field}>
                    <label htmlFor="password" className={styles.label}>
                        Password:
                    </label>
                    <input
                    type="password"
                    id="password"
                    value={password}
                    onChange={handleInputChange}
                    className={styles.input}
                    />
                </div>
                <div className={styles.linkContainer}>
                    <a href='/register' className={styles.link}>
                        Don't have an account? Register here
                    </a>
                </div>
            </form>
        </div>
    )
}

export default Login;
