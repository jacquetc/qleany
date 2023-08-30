// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "close_system_command_handler.h"
#include "qleany/tools/automapper/automapper.h"

#include <QDebug>

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Application::Features::Custom::Commands;

CloseSystemCommandHandler::CloseSystemCommandHandler(InterfaceCarRepository *carRepository,
                                                     InterfacePassengerRepository *passengerRepository,
                                                     InterfaceBrandRepository *brandRepository,
                                                     InterfaceClientRepository *clientRepository)
    : m_carRepository(carRepository), m_passengerRepository(passengerRepository), m_brandRepository(brandRepository),
      m_clientRepository(clientRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> CloseSystemCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                               const CloseSystemCommand &request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CloseSystemCommand:" << ex.what();
    }
    return result;
}

Result<void> CloseSystemCommandHandler::restore()
{

    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> CloseSystemCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const CloseSystemCommand &request)
{
    qDebug() << "CloseSystemCommandHandler::handleImpl called";

    // implement logic here which will not be repeated on restore
    // custom = Qleany::Tools::AutoMapper::AutoMapper::map<void, Simple::Domain::Custom>(request.req);

    m_carRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_carRepository->saveChanges();

    // emit signal
    // emit closeSystemChanged();

    // Return
    return Result<void>();
}

bool CloseSystemCommandHandler::s_mappingRegistered = false;

void CloseSystemCommandHandler::registerMappings()
{
}