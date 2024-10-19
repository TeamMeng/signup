import { useEffect, useState } from 'react';
import styles from './Profile.module.css'
import axios from 'axios';
import router from 'next/router';

interface ProfileDetails {
    name: string;
    email: string;
}

const Profile: React.FC = () => {
    const [profile, setProfile] = useState<ProfileDetails  | null>(null);

    useEffect(() => {
        const fetchProfile = async () => {
            try {
                const response = await axios.get<ProfileDetails>('http://localhost:8080/profile', {
                    withCredentials: true
                });
                if (response.status === 200) {
                    setProfile(response.data);
                } else {
                    await router.push('/');
                }
            } catch (error) {
                await router.push('/');
            }
        };

        fetchProfile();
    }, [router]);

    return (
        <div className={styles.profileContainer}>
            {
                profile? (
                    <div>
                        <h1>Profile</h1>
                        <p>Name: {profile.name}</p>
                        <p>Email: {profile.email}</p>
                    </div>
                ) : (
                    <p className={styles.loading}>Loading profile...</p>
                )
            }
        </div>
    )
}
