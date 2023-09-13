import numpy as np
def system( t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    A,B,C, = y

    


    dA = +(A  * B )
    dB = -(A  * B )
    dC = +(A  * B  / C )

    return np.array([dA, dB, dC, ])