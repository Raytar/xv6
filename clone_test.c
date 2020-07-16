#include "types.h"
#include "user.h"

void thread(void *arg1, void *arg2) {
	int my_pid = getpid();
	printf(1, "Thread (pid %d) started from main (pid %d)\n", my_pid, *(int *)arg1);
	exit();
}

int main(int argc, char *argv[]) {
	void *thr_stack = malloc(1024);
	int pid = getpid();
	clone(thread, &pid, 0, thr_stack);
	sleep(100);
	printf(1, "Main thread exit");
	exit();
}
