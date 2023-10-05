import numpy as np
def system( t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    X,Y, = y

    
    delta,omega,alpha,beta, = constants
    


    dX = +(X  * Y  * delta )+(X  * Y  * beta )+(X  * alpha )
    dY = +(Y  * omega )+(X  * Y  * beta )+(X  * Y  * delta )

    return np.array([dX, dY, ])