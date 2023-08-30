// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "write_random_things_command_handler.h"
#include "custom/validators/write_random_things_command_validator.h"
#include "custom/write_random_things_dto.h"
#include "qleany/tools/automapper/automapper.h"
#include <QDebug>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Custom;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Custom::Validators;
using namespace Simple::Application::Features::Custom::Commands;

WriteRandomThingsCommandHandler::WriteRandomThingsCommandHandler(InterfaceCarRepository *carRepository,
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

Result<void> WriteRandomThingsCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const WriteRandomThingsCommand &request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling WriteRandomThingsCommand:" << ex.what();
    }
    return result;
}

Result<void> WriteRandomThingsCommandHandler::restore()
{

    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> WriteRandomThingsCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                         const WriteRandomThingsCommand &request)
{
    qDebug() << "WriteRandomThingsCommandHandler::handleImpl called";

    // Validate the create custom command using the validator
    auto validator = WriteRandomThingsCommandValidator(m_carRepository, m_passengerRepository, m_brandRepository,
                                                       m_clientRepository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {

        return Result<void>(validatorResult.error());
    }

    // implement logic here which will not be repeated on restore
    // custom = Qleany::Tools::AutoMapper::AutoMapper::map<WriteRandomThingsDTO, Simple::Domain::Custom>(request.req);

    m_carRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_carRepository->saveChanges();

    // emit signal
    // emit writeRandomThingsChanged();

    // Return
    return Result<void>();
}

bool WriteRandomThingsCommandHandler::s_mappingRegistered = false;

void WriteRandomThingsCommandHandler::registerMappings()
{
}