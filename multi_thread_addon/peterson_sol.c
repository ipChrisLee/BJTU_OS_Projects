#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <signal.h>
#include <unistd.h>
#include <stdbool.h>

void mfence() {
	__sync_synchronize();
}

struct PetersonControl {
	volatile bool c[2];
	volatile int turn;
};
struct PetersonControl * petersonControl;

void init(struct PetersonControl ** pThis) {
	*pThis = calloc(sizeof(struct PetersonControl), 1);
}

void acquire(struct PetersonControl * this, int id) {
	this->c[id] = true;
	this->turn = 1 - id;
	mfence();
	while (this->c[1 - id] && this->turn == 1 - id) {
	}
}

void release(struct PetersonControl * this, int id) {
	mfence();
	this->c[id] = false;
}

const int MX_CNT = (int) 1e7;
int acc1, acc2, err[3];
int c[2], turn;

void * give_and_get(void * pInt) {
	int idx = *(int *) pInt, cnt = 0;
	do {
		acquire(petersonControl, idx);
		int r = rand();
		acc1 = acc1 + r;
		acc2 = acc2 - r;
		if (acc1 + acc2 != 0) {
			++err[idx];
			release(petersonControl, idx);
			break;
		}
		
		release(petersonControl, idx);
	} while (++cnt <= MX_CNT);
	return NULL;
}

int main() {
	struct timespec tsBegin, tsEnd;
	clock_gettime(CLOCK_MONOTONIC, &tsBegin);
	init(&petersonControl);
	pthread_t t[2];
	int idx[2] = {0, 1};
	for (int i = 0; i < 2; ++i) {
		pthread_create(&t[i], NULL, give_and_get, &idx[i]);
	}
	for (int i = 0; i < 2; ++i) {
		pthread_join(t[i], NULL);
	}
	clock_gettime(CLOCK_MONOTONIC, &tsEnd);
	printf("%lf\n", (double) (tsEnd.tv_sec - tsBegin.tv_sec) +
		(double) (tsEnd.tv_nsec - tsBegin.tv_nsec) / 1e9
	);
	return err[1] + err[2];
}
