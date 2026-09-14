# include <math.h>
# include <vector>
# include <iostream>
# include <string>
# include <unistd.h>

# define SCREEN_WIDTH 338
# define SCREEN_HEIGHT 91

class SpinningCube {
private:
	float rotateX, rotateY, rotateZ = 0;
	float posX, posY, posZ; // position
	float cubeWidth 	= 20;
	std::vector<float>	zBuffer;
	std::vector<char>	buffer;

	int distanceFromCam = 60;
	int K1 = 40;
	float incrementSpeed = 0.3;
	
	float calculateX(int i ,int j, int k) {
		return	j * sin(rotateX) * sin(rotateY) * cos(rotateZ) - 
				k * cos(rotateX) * sin(rotateY) * cos(rotateZ) +
				j * cos(rotateX) * sin(rotateZ) +
				k * sin(rotateX) * sin(rotateZ) +
				i * cos(rotateY) * cos(rotateZ);
	}

	float calculateY(int i ,int j, int k) {
		return	j * cos(rotateX) * cos(rotateZ) +
				k * sin(rotateX) * cos(rotateZ) - 
				j * sin(rotateX) * sin(rotateY) * sin(rotateZ) +
				k * cos(rotateX) * sin(rotateY) * sin(rotateZ) -
				i * cos(rotateY) * sin(rotateZ);
	}

	float calculateZ(int i, int j, int k) {
		return	k * cos(rotateX) * cos(rotateY) -
				j * sin(rotateX) * cos(rotateY) + 
				i * sin(rotateY);
	}

	void calculateSurface(float cubeX, float cubeY, float cubeZ, float normalX, float normalY, float normalZ) {
		posX = calculateX(cubeX, cubeY, cubeZ);
		posY = calculateY(cubeX, cubeY, cubeZ);
		posZ = calculateZ(cubeX, cubeY, cubeZ) + distanceFromCam;


		float ooz = 1 / posZ;
		int screenPosX = (int)(SCREEN_WIDTH / 2 + K1 * ooz * posX * 2);
		int screenPosY = (int)(SCREEN_HEIGHT / 2 + K1 * ooz * posY);

		int index = screenPosX + screenPosY * SCREEN_WIDTH;
		if (index >= 0 && index < SCREEN_WIDTH * SCREEN_HEIGHT) {
			if (ooz > zBuffer[index]) {
				float luminance = calculateLuminance(normalX, normalY, normalZ);
				
                float L = (luminance > 0.0f) ? luminance : 0.0f; // Clamp negative luminance to 0.0 (shadowed faces)

                int luminanceIndex = (int)(L * 10.0f); // Map 0.0 -> 1.0 safely across 11 characters (indices 0 to 10)
                if (luminanceIndex > 10) luminanceIndex = 10;

				zBuffer[index] = ooz;
				buffer[index] = ".,-~:;=*#$@"[luminanceIndex];
			}
		}
	}

	void clearBuffers() {
		zBuffer.assign(zBuffer.size(), 0);
		buffer.assign(buffer.size(), ' ');
	}

	void printCube() {
		for (int i = 0; i < SCREEN_WIDTH * SCREEN_HEIGHT; i++) {
			std::cout << (i % SCREEN_WIDTH != 0 ? buffer[i] : '\n');
		}
	}

	float calculateLuminance(float normalX, float normalY, float normalZ) {
		float lightX = 0.0f;
		float lightY = 0.7071f;
		float lightZ = -0.7071f;
		float normX = calculateX(normalX, normalY, normalZ);
        float normY = calculateY(normalX, normalY, normalZ);
        float normZ = calculateZ(normalX, normalY, normalZ);
		return (normalX * lightX) + (normalY * lightY) + (normalZ * lightZ);
	}

public:
	SpinningCube(): zBuffer(SCREEN_WIDTH * SCREEN_HEIGHT), buffer(SCREEN_WIDTH * SCREEN_HEIGHT) {}

	void spin() {
		while (true) {
			clearBuffers();
			for (float cubeX = -cubeWidth; cubeX < cubeWidth; cubeX += incrementSpeed) { // idk what calculation
				for (float cubeY = -cubeWidth; cubeY < cubeWidth; cubeY += incrementSpeed) {
					calculateSurface(cubeX, cubeY, cubeWidth,   0,  0,  1); // Front  (0, 0, 1)
                    calculateSurface(-cubeWidth, cubeY, cubeX,  -1,  0,  0); // Left   (-1, 0, 0)
                    calculateSurface(cubeWidth, cubeY, -cubeX,   1,  0,  0); // Right  (1, 0, 0)
                    calculateSurface(cubeX, cubeY, -cubeWidth,  0,  0, -1); // Back   (0, 0, -1)
                    calculateSurface(cubeX, cubeWidth, -cubeY,  0,  1,  0); // Top    (0, 1, 0)
                    calculateSurface(cubeX, -cubeWidth, cubeY,  0, -1,  0); // Bottom (0, -1, 0)
				}
			}

			printCube();
			
			rotateX += 0.005;
			rotateY += 0.005;
			rotateZ += 0.005;
			usleep(1500);
		}
	}
};


int main(void) {
	SpinningCube spinningCube;
	spinningCube.spin();

	return 0;
}
