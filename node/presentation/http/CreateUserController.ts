import { Request, Response } from 'express';
import { CreateUser } from '../../application/usecases/CreateUser';

export const createUserController = (createUserUseCase: CreateUser) => 
    async (req: Request, res: Response) => {
        try {
            const user = await createUserUseCase.execute(req.body);
            res.status(201).json(user);
        } catch (e: any) {
            res.status(400).json({ error: e.message });
        }
    };