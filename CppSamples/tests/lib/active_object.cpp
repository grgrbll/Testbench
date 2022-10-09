#include <thread>
#include <future>
#include <queue>
#include <mutex>
#include <memory>
#include <optional>

class ActiveObject
{
public:
    class Operation
    {
    public:
        virtual void DoWork(){};
        virtual ~Operation(){};
    };

    std::queue<std::shared_ptr<Operation>> myQueue;
    mutable std::mutex m;
    std::condition_variable condVar;
    std::thread t;
    bool shutdown;

    void Shutdown()
    {
        {
            std::scoped_lock<std::mutex> lock(m);

            if (shutdown)
                return;

            shutdown = true;
            condVar.notify_one();
        }
        t.join();
    }

    void PutOperation(std::shared_ptr<Operation> op)
    {
        std::scoped_lock<std::mutex> lock(m);
        myQueue.push(op);
        condVar.notify_one();
    }

    std::optional<std::shared_ptr<Operation>> GetOperation()
    {
        std::unique_lock<std::mutex> lock(m);

        while (myQueue.empty() && !shutdown)
            condVar.wait(lock);

        if (shutdown)
            return std::nullopt;

        auto op = myQueue.front();
        myQueue.pop();
        return op;
    }

    void ProcessThread()
    {
        while (!shutdown)
        {
            std::optional<std::shared_ptr<Operation>> op = GetOperation();
            if (op)
            {
                op.value()->DoWork();
            }
        }
    }

    ActiveObject() : myQueue(), m(), condVar(), shutdown(false)
    {
        t = std::thread(&ActiveObject::ProcessThread, this);
    }

    class Add : public Operation
    {
        int a;
        int b;
        std::promise<int> result;

    public:
        Add(int a, int b, std::promise<int> &&res) : a(a), b(b), result(std::move(res)) {}

        virtual void DoWork()
        {
            result.set_value(a + b);
        }
    };

    int RequestAdd(int a, int b)
    {
        std::promise<int> p;
        auto f = p.get_future();
        PutOperation(std::make_shared<Add>(a, b, std::move(p)));
        return f.get();
    }
};