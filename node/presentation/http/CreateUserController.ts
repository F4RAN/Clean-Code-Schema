import { Request, Response } from 'express';
import { CreateUser } from '../../application/usecases/CreateUser';

/**
 * This is a higher-order function (function that returns a function)
 * 
 * STEP 1: You call this with useCase (setup time)
 *   const handler = createUserController(useCase);
 *   - At this point: req and res DON'T EXIST YET
 *   - We're just creating the handler function
 *   - Returns: the inner function (Express handler)
 * 
 * STEP 2: Express calls the returned handler (request time)
 *   app.post('/users', handler);
 *   - When POST /users is hit, Express automatically calls: handler(req, res)
 *   - At this point: req and res are created by Express and passed in
 */
export const createUserController = (createUserUseCase: CreateUser) => 
    // This inner function is what Express will call
    // Express provides req and res when it calls this function
    async (req: Request, res: Response) => {
        try {
            // req and res are available here because Express called this function
            // with: handler(req, res)
            const user = await createUserUseCase.execute(req.body);
            res.status(201).json(user);
        } catch (e: any) {
            res.status(400).json({ error: e.message });
        }
    };


