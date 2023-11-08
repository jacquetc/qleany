// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "run_long_operation_command_handler.h"
#include <qleany/tools/automapper/automapper.h>

#include <QDebug>

using namespace Qleany;

using namespace Simple::Application::Features::Custom::Commands;

RunLongOperationCommandHandler::RunLongOperationCommandHandler()

{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> RunLongOperationCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                    const RunLongOperationCommand &request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RunLongOperationCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<void> RunLongOperationCommandHandler::restore()
{

    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> RunLongOperationCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                        const RunLongOperationCommand &request)
{
    qDebug() << "RunLongOperationCommandHandler::handleImpl called";

    // mock a long async operation that reports progress
    progressPromise.setProgressRange(0, 100);
    progressPromise.setProgressValueAndText(50, QString("Imitating a long operation %1/100").arg(50));
    int progress = 0;

    while (progress < 100)
    {
        progress += 10;
        // progressPromise.setProgressValueAndText(progress, QString("Imitating a long operation
        // %1/100").arg(progress));
        //  mock big operation
        for (int i = 0; i < 100000000; ++i)
        {
            // do nothing
        }
    }
    qDebug() << "RunLongOperationCommandHandler::handleImpl finished";

    // Return
    return Result<void>();
}

bool RunLongOperationCommandHandler::s_mappingRegistered = false;

void RunLongOperationCommandHandler::registerMappings()
{
}
