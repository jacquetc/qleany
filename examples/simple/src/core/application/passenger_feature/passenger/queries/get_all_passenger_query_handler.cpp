// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_passenger_query_handler.h"
#include "repository/interface_passenger_repository.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Application::Features::Passenger::Queries;

GetAllPassengerQueryHandler::GetAllPassengerQueryHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<PassengerDTO>> GetAllPassengerQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllPassengerQueryHandler::handle called";

    Result<QList<PassengerDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<PassengerDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllPassengerQuery:" << ex.what();
    }
    return result;
}

Result<QList<PassengerDTO>> GetAllPassengerQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllPassengerQueryHandler::handleImpl called";

    // do
    auto passengerResult = m_repository->getAll();

    if (Q_UNLIKELY(passengerResult.isError()))
    {
        return Result<QList<PassengerDTO>>(passengerResult.error());
    }

    // map
    QList<PassengerDTO> dtoList;

    for (const Simple::Domain::Passenger &passenger : passengerResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(passenger);
        dtoList.append(dto);
    }

    qDebug() << "GetAllPassengerQueryHandler::handleImpl done";

    return Result<QList<PassengerDTO>>(dtoList);
}

bool GetAllPassengerQueryHandler::s_mappingRegistered = false;

void GetAllPassengerQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true);
}