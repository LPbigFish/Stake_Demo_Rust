import { createClient } from '@supabase/supabase-js';
import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY } from '$env/static/public'

export const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY);

export const login = async (email: string, password: string) => {
    const { error } = await supabase.auth.signInWithPassword({
        email,
        password,
    })

    return {data: !error, error};
};

export const login_with_discord = async () => {
    const { error } = await supabase.auth.signInWithOAuth({
        provider: 'discord',
    })

    return {data: !error, error};
};

export const get_user = async () => {
    return supabase.auth.getUser();
}

export const logout = async () => {
    return supabase.auth.signOut();
}

export const signin = async (email: string, password: string) => {
    const { error } = await supabase.auth.signUp({
        email,
        password,
    })

    return {data: !error, error};
}

export const set_new_username = async (username: string) => {
    if ((await supabase.from('users').select().eq('uuid', (await get_user())?.data.user?.id)).count === 0) {
        return await supabase.from('users').insert([{uuid: (await get_user())?.data.user?.id, username: username}]);
    }
    return await supabase.from('users').update({username: username}).eq('uuid', (await get_user())?.data.user?.id);
}