// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "close_system_command_handler.h"
#include "tools/automapper.h"

#include <QDebug>

using namespace FrontEnds;

using namespace FrontEnds::Contracts::Repository;

using namespace FrontEnds::Application::Features::Custom::Commands;

CloseSystemCommandHandler::CloseSystemCommandHandler(InterfaceCarRepository *carRepository,
                                                     InterfacePassengerRepository *passengerRepository,
                                                     InterfaceBrandRepository *brandRepository,
                                                     InterfaceClientRepository *clientRepository)
    : m_carRepository(carRepository)
    , m_passengerRepository(passengerRepository)
    , m_brandRepository(brandRepository)
    , m_clientRepository(clientRepository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> CloseSystemCommandHandler::handle(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request)
{
    Result<void> result;

    try {
        result = handleImpl(progressPromise, request);
    } catch (const std::exception &ex) {
        result = Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CloseSystemCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<void> CloseSystemCommandHandler::restore()
{
    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> CloseSystemCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request)
{
    qDebug() << "CloseSystemCommandHandler::handleImpl called";

    // implement logic here which will not be repeated on restore
    // custom = FrontEnds::Tools::AutoMapper::map<void, FrontEnds::Entities::Custom>(request.req);

    m_carRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_carRepository->saveChanges();

    // Q_EMIT signal
    // Q_EMIT closeSystemChanged();

    // Return
    return Result<void>();
}

bool CloseSystemCommandHandler::s_mappingRegistered = false;

void CloseSystemCommandHandler::registerMappings()
{
}