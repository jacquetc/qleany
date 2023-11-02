// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_brand_query_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_brand_repository.h"

using namespace Qleany;
using namespace Simple::Application::Features::Brand::Queries;

GetBrandQueryHandler::GetBrandQueryHandler(InterfaceBrandRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BrandDTO> GetBrandQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetBrandQuery &query)
{
    Result<BrandDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<BrandDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetBrandQuery:" << ex.what();
    }
    return result;
}

Result<BrandDTO> GetBrandQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetBrandQuery &query)
{
    qDebug() << "GetBrandQueryHandler::handleImpl called with id" << query.id;

    // do
    auto brandResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(BrandDTO, brandResult)

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(brandResult.value());

    qDebug() << "GetBrandQueryHandler::handleImpl done";

    return Result<BrandDTO>(dto);
}

bool GetBrandQueryHandler::s_mappingRegistered = false;

void GetBrandQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Brand, Contracts::DTO::Brand::BrandDTO>(
        true, true);
}