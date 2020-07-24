#include "types.h"
#include "user.h"

void thread(void *arg1, void *arg2) {
	int my_pid = getpid();
	printf(1, "Thread (pid %d) started from main (pid %d)\n", my_pid, *(int *)arg1);
	exit();
}

int main(int argc, char *argv[]) {
	void *thr_stack = malloc(4096);
	if (!thr_stack) {
		printf(2, "malloc error\n");
		exit();
	}
	// pointer to bottom of stack (= highest address in stack)
	void *stack_bottom = thr_stack + 4096;
	int pid = getpid();
	if (clone(thread, &pid, 0, stack_bottom)) {
		printf(2, "clone error\n");
	}
	sleep(100);
	printf(1, "Main thread exit\n");
	exit();
}
